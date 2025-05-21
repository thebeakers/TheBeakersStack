<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onMount, onDestroy } from 'svelte'; // onDestroy added
	import { githubTokenStore } from '$lib/stores'; // Import the store

	let verificationUri = $state(''); // Svelte 5
	let userCode = $state(''); // Svelte 5

	let authStep = $state<'initial' | 'awaiting_user_action' | 'authenticated' | 'error'>('initial'); // Svelte 5
	let authError = $state(''); // Svelte 5
	let isLoading = $state(false); // Svelte 5

	let unlistenStartedFn: UnlistenFn | null = null;
	let unlistenRespondedFn: UnlistenFn | null = null;

	onMount(() => {
		console.log('LoginGit.svelte: onMount - component mounted');

		(async () => {
			try {
				console.log('LoginGit.svelte: Listening for auth events (async setup)');

				unlistenStartedFn = await listen('auth-started', (event) => {
					console.log(
						'LoginGit.svelte EVENT: auth-started. Current authStep:',
						authStep,
						'Payload:',
						event.payload
					);
					if (event.payload) {
						const payload = event.payload as {
							verification_uri: string;
							user_code: string;
						};
						if (payload.verification_uri && payload.user_code) {
							verificationUri = payload.verification_uri;
							userCode = payload.user_code;
							// authStep will be set to 'awaiting_user_action' in onSubmit
						}
					}
				});

				unlistenRespondedFn = await listen('auth-responded', (event) => {
					console.log(
						'LoginGit.svelte EVENT: auth-responded. Current authStep:',
						authStep,
						'Payload:',
						event.payload
					);
					isLoading = false;
					if (event.payload && typeof event.payload === 'string' && event.payload !== '') {
						const receivedToken = event.payload as string;
						githubTokenStore.set(receivedToken); // <--- UPDATE THE STORE
						authStep = 'authenticated';
						verificationUri = '';
						userCode = '';
						authError = '';
						console.log('LoginGit.svelte: auth-responded: Success! authStep set to authenticated.');
					} else {
						authError = 'Authentication failed or polling timed out. Please try again.';
						authStep = 'error';
						githubTokenStore.set(null); // <--- Clear the store on error
						console.error(
							'LoginGit.svelte: auth-responded: Failure or empty payload. authStep set to error. Payload:',
							event.payload
						);
					}
				});
				console.log('LoginGit.svelte: Event listeners attached.');
			} catch (error) {
				console.error('LoginGit.svelte: Error setting up event listeners:', error);
				authError = 'Failed to initialize system. Please refresh.';
				authStep = 'error';
				isLoading = false;
			}
		})();

		// Return the cleanup function synchronously
		return () => {
			console.log('LoginGit.svelte: onMount - cleaning up listeners');
			if (unlistenStartedFn) {
				unlistenStartedFn();
				console.log('LoginGit.svelte: Unlistened from auth-started.');
			}
			if (unlistenRespondedFn) {
				unlistenRespondedFn();
				console.log('LoginGit.svelte: Unlistened from auth-responded.');
			}
		};
	});

	// Explicit onDestroy for clarity, though onMount's return function handles it.
	onDestroy(() => {
		if (unlistenStartedFn) unlistenStartedFn();
		if (unlistenRespondedFn) unlistenRespondedFn();
	});

	const onSubmit = async () => {
		if (isLoading) return;

		console.log('LoginGit.svelte: onSubmit: Initiating new auth attempt.');
		isLoading = true;
		authStep = 'initial';
		verificationUri = '';
		userCode = '';
		githubTokenStore.set(null); // Clear token at start of new attempt
		authError = '';

		try {
			const startAuthPayload = await invoke<{
				verification_uri: string;
				user_code: string;
			}>('start_auth');
			console.log('LoginGit.svelte: onSubmit: start_auth invoked. Payload:', startAuthPayload);

			if (startAuthPayload && startAuthPayload.verification_uri && startAuthPayload.user_code) {
				verificationUri = startAuthPayload.verification_uri;
				userCode = startAuthPayload.user_code;
				authStep = 'awaiting_user_action';
				console.log(
					'LoginGit.svelte: onSubmit: Set authStep to awaiting_user_action. URI:',
					verificationUri,
					'Code:',
					userCode
				);

				console.log('LoginGit.svelte: onSubmit: Invoking wait_for_auth to start polling...');
				// The 'auth-responded' event is the primary handler for the token.
				// This invoke call's resolution mainly signals completion or direct error from polling setup.
				invoke('wait_for_auth')
					.then((receivedTokenIfSuccessful) => {
						console.log(
							'LoginGit.svelte: invoke(wait_for_auth) promise resolved.',
							'Token from promise (event is primary):',
							receivedTokenIfSuccessful
						);
						// If authStep is still 'awaiting_user_action', it means the event might not have fired
						// or this resolved first. We can try to set auth state here as a fallback.
						if (authStep === 'awaiting_user_action') {
							if (
								typeof receivedTokenIfSuccessful === 'string' &&
								receivedTokenIfSuccessful !== ''
							) {
								githubTokenStore.set(receivedTokenIfSuccessful); // Ensure store is set
								authStep = 'authenticated';
							} else if (!githubTokenStore) {
								// If store is still null
								authError = 'Polling completed but no valid token was confirmed.';
								authStep = 'error';
							}
						}
						isLoading = false; // Ensure loading is stopped
					})
					.catch((pollingError) => {
						console.error(
							'LoginGit.svelte: Error invoking or during wait_for_auth polling:',
							pollingError
						);
						if (authStep !== 'authenticated') {
							authError =
								(pollingError as Error)?.message || 'Polling for authentication token failed.';
							authStep = 'error';
							githubTokenStore.set(null);
						}
						isLoading = false;
					});
			} else {
				throw new Error('Invalid payload received from start_auth command');
			}
		} catch (e: any) {
			console.error('LoginGit.svelte: onSubmit: Error during start_auth invoke or setup:', e);
			authError = e.message || 'Failed to start GitHub authentication. Please try again.';
			authStep = 'error';
			isLoading = false;
			githubTokenStore.set(null);
		}
	};
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-inherit p-4">
	{#if authStep === 'initial' || authStep === 'error'}
		<div class="w-96 rounded bg-white p-8 shadow-md">
			<h2 class="mb-6 text-center text-2xl font-semibold">Log in with GitHub</h2>
			<button
				class="flex w-full items-center justify-center rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
				on:click={onSubmit}
				disabled={isLoading}
			>
				{#if isLoading && authStep === 'initial'}
					<svg
						class="mr-2 h-4 w-4 animate-spin text-white"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					<span>Starting...</span>
				{:else}
					<svg
						class="mr-2 h-4 w-4 fill-current"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
					>
						<path
							d="M12 0C5.373 0 0 5.373 0 12c0 5.31 3.435 9.8 8.205 11.385.6.11.82-.255.82-.575 0-.28-.01-1.02-.01-2.025-3.34.73-4.04-1.615-4.04-1.615-.54-1.38-1.33-1.75-1.33-1.75-1.09-.745.08-.73.08-.73 1.205.085 1.84 1.24 1.84 1.24 1.075 1.825 2.82 1.3 3.505.995.105-.765.42-1.295.765-1.6-2.675-.3-5.49-1.335-5.49-5.93 0-1.315.47-2.39 1.235-3.225-.12-.3-.53-1.52.115-3.175 0 0 1.005-.325 3.3 1.23.96-.27 1.995-.405 3.02-.41 1.025.005 2.06.135 3.015.405 2.3-.555 3.295-1.23 3.295-1.23.645 1.655.23 2.87.115 3.175.77.835 1.23 1.91 1.23 3.225 0 4.605-2.815 5.62-5.5 5.925.43.375.81 1.1.81 2.23 0 1.615-.01 2.915-.01 3.315 0 .32.215.69.82.57C20.565 21.8 24 17.305 24 12 24 5.373 18.627 0 12 0z"
						/>
					</svg>
					<span>Continue with GitHub</span>
				{/if}
			</button>
			{#if authStep === 'error' && authError}
				<p class="mt-4 text-center text-sm text-destructive">
					{authError}
				</p>
			{/if}
		</div>
	{/if}

	{#if authStep === 'awaiting_user_action'}
		<div
			class="mt-6 w-96 rounded-lg border border-border bg-card p-6 text-card-foreground shadow-lg"
		>
			<h3 class="mb-4 text-center text-xl font-semibold">Complete Authentication</h3>
			{#if verificationUri}
				<div class="mb-4">
					<p class="mb-1 text-sm font-medium text-foreground">1. Open this URL in your browser:</p>
					<a
						href={verificationUri}
						target="_blank"
						rel="noopener noreferrer"
						class="block break-all rounded-md bg-muted p-3 font-mono text-sm text-primary hover:underline focus:outline-none focus:ring-2 focus:ring-ring"
					>
						{verificationUri}
					</a>
				</div>
			{/if}
			{#if userCode}
				<div>
					<p class="mb-1 text-sm font-medium text-foreground">2. Enter this code:</p>
					<div class="select-all rounded-md bg-muted p-3 text-center">
						<strong class="font-mono text-2xl font-bold tracking-wider text-primary"
							>{userCode}</strong
						>
					</div>
				</div>
			{/if}
			<div class="mt-6 text-center">
				<svg
					class="mx-auto h-8 w-8 animate-spin text-primary"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					></path>
				</svg>
				<p class="mt-2 text-sm text-muted-foreground">Waiting for you to authorize on GitHub...</p>
			</div>
		</div>
	{/if}

	{#if authStep === 'authenticated'}
		<div
			class="mt-6 w-96 rounded-lg border border-border bg-card p-6 text-card-foreground shadow-lg"
		>
			<h3 class="mb-4 text-center text-xl font-semibold text-green-600">
				Successfully Authenticated!
			</h3>
			<p class="text-center">You can now proceed to the editor.</p>
			<div class="mt-6 text-center">
				<button
					class="rounded bg-green-500 px-6 py-2 font-bold text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-400 focus:ring-opacity-75"
					on:click={async () => {
						await goto('/edit_toml');
					}}
				>
					Go to Edit Page
				</button>
			</div>
		</div>
	{/if}
</div>
