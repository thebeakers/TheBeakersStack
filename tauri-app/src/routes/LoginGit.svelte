<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	let token = '';
	let userCode = '';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let unlisten: UnlistenFn;
	onMount(async () => {
		console.log('Listening to `auth-responded`');

		unlisten = await listen('auth-responded', (event) => {
			console.log('EVENT RECEIVED!!');
			console.log(event);
			token = event.payload as string;
		});
	});

	const onSubmit = async () => {
		let x = await invoke('start_auth').catch((e) => {
			console.error('Error during auth:', e);
		});
		await invoke('wait_for_auth').catch((e) => {
			console.error('Error during auth:', e);
		});
	};
</script>

<div class="flex h-screen items-center justify-center bg-inherit">
	<div class="w-96 rounded bg-white p-8 shadow-md">
		<h2 class="mb-6 text-center text-2xl font-semibold">Log in with GitHub</h2>
		<button
			class="flex w-full items-center justify-center rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
			on:click={onSubmit}
		>
			<svg class="mr-2 h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path
					d="M12 0C5.373 0 0 5.373 0 12c0 5.31 3.435 9.8 8.205 11.385.6.11.82-.255.82-.575 0-.28-.01-1.02-.01-2.025-3.34.73-4.04-1.615-4.04-1.615-.54-1.38-1.33-1.75-1.33-1.75-1.09-.745.08-.73.08-.73 1.205.085 1.84 1.24 1.84 1.24 1.075 1.825 2.82 1.3 3.505.995.105-.765.42-1.295.765-1.6-2.675-.3-5.49-1.335-5.49-5.93 0-1.315.47-2.39 1.235-3.225-.12-.3-.53-1.52.115-3.175 0 0 1.005-.325 3.3 1.23.96-.27 1.995-.405 3.02-.41 1.025.005 2.06.135 3.015.405 2.3-.555 3.295-1.23 3.295-1.23.645 1.655.23 2.87.115 3.175.77.835 1.23 1.91 1.23 3.225 0 4.605-2.815 5.62-5.5 5.925.43.375.81 1.1.81 2.23 0 1.615-.01 2.915-.01 3.315 0 .32.215.69.82.57C20.565 21.8 24 17.305 24 12 24 5.373 18.627 0 12 0z"
				/>
			</svg>
			Continue with GitHub
		</button>
	</div>
</div>

<div>
	<button
		on:click={async () => {
			await goto('/edit_toml');
		}}>Go to the edit</button
	>
</div>

<div>
	{userCode}
	{token}
</div>
