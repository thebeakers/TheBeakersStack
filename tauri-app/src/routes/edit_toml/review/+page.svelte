<script lang="ts">
	import { articleStore, githubTokenStore } from '$lib/stores';
	import type { Article } from '$lib/types';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';

	// Use $state for local component state that changes and triggers UI updates
	let isUploading = $state(false);
	let uploadMessage = $state('');
	let uploadError = $state(false);

	// Svelte 5 allows direct reactive use of store values: $articleStore, $githubTokenStore
	// No need for manual subscriptions and local variables like `article` and `ghToken`
	// if they are just mirrors of the store.

	function formatDateString(dateString: string | null | undefined): string {
		if (!dateString || dateString.trim() === '') {
			return 'Not set';
		}
		try {
			return new Date(dateString).toLocaleDateString(undefined, {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
		} catch (e) {
			console.error('Invalid date string for formatting:', dateString, e);
			return 'Invalid Date';
		}
	}

	function slugify(text: string): string {
		if (!text) return 'untitled-article';
		return text
			.toString()
			.toLowerCase()
			.trim()
			.replace(/\s+/g, '-')
			.replace(/[^\w-]+/g, '')
			.replace(/--+/g, '-')
			.replace(/^-+/, '')
			.replace(/-+$/, '');
	}

	const handleUploadToGitHub = async () => {
		const currentArticle = $articleStore; // Get current value from store reactively

		if (!currentArticle) {
			uploadMessage = 'Error: No article data to upload.';
			uploadError = true;
			return;
		}

		const currentGhToken = $githubTokenStore; // Get current value from store reactively
		if (!currentGhToken) {
			uploadMessage = 'Error: GitHub token is missing. Please log in again.';
			uploadError = true;
			return;
		}

		isUploading = true;
		uploadMessage = 'Preparing to upload...';
		uploadError = false;

		try {
			const fileName = `${slugify(currentArticle.title)}.toml`;

			uploadMessage = `Uploading article '${currentArticle.title}' to GitHub as ${fileName}...`;
			console.log(`Preparing to upload: ${fileName}`);
			console.log('Article data being sent to Rust:', JSON.parse(JSON.stringify(currentArticle)));

			const result = await invoke<string>('upload_article_to_github', {
				article: currentArticle,
				fileName: fileName
			});

			uploadMessage = `Success: ${result}`;
			uploadError = false;
		} catch (error: any) {
			console.error('Error uploading to GitHub:', error);
			const errorMessage =
				typeof error === 'string' ? error : error?.message || JSON.stringify(error);
			uploadMessage = `Failed to upload: ${errorMessage}`;
			uploadError = true;
		} finally {
			isUploading = false;
		}
	};
</script>

<main class="min-h-screen bg-gray-800 p-6 font-serif text-gray-100">
	{#if $articleStore}
		<!-- Directly use store value for conditional rendering -->
		<div class="mx-auto mb-8 max-w-4xl text-center">
			<h1 class="mb-4 text-3xl font-bold text-white">Does this look good?</h1>
			<p class="text-gray-300">
				Review the article details below. If everything is correct, you can proceed.
			</p>
			<div class="mt-6 flex flex-wrap justify-center gap-4">
				<Button
					variant="outline"
					onclick={() => {
						goto('/edit_toml');
					}}
					disabled={isUploading}
				>
					Go Back & Edit
				</Button>
				<Button
					variant="secondary"
					onclick={handleUploadToGitHub}
					disabled={isUploading || !$githubTokenStore}
					title={!$githubTokenStore
						? 'Please log in with GitHub first'
						: 'Upload to GitHub repository'}
				>
					{#if isUploading}
						<svg
							class="mr-2 h-4 w-4 animate-spin"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Uploading...
					{:else}
						Upload to GitHub
					{/if}
				</Button>
			</div>
			<!-- Check against the store value directly -->
			{#if !$githubTokenStore && !isUploading}
				<p class="mt-2 text-sm text-yellow-400">
					GitHub token not found. Please
					<a href="/" class="underline hover:text-yellow-300">log in via GitHub</a>
					to enable uploads.
				</p>
			{/if}
			{#if uploadMessage}
				<p
					class="mt-4 text-sm"
					class:text-green-400={!uploadError}
					class:text-red-400={uploadError}
				>
					{uploadMessage}
				</p>
			{/if}
		</div>

		<!-- Header Section (Article Display, using $articleStore directly) -->
		<div class="mx-auto max-w-4xl">
			<div class="mb-4">
				<img
					src={$articleStore.image.url}
					alt={$articleStore.image.alt}
					class="h-auto w-full rounded-md object-cover"
				/>
				<p class="mt-2 text-center text-sm italic text-gray-400">
					{$articleStore.image.caption}
				</p>
			</div>
			<h1 class="mb-2 text-4xl font-bold">{$articleStore.title}</h1>
			<p class="mb-4 text-lg text-gray-300">{$articleStore.description}</p>
			<div class="mb-2 text-sm text-gray-400">
				<span class="font-semibold">Category:</span>
				{$articleStore.category ?? 'N/A'}
			</div>
			<div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-gray-400">
				{#if $articleStore.authors && $articleStore.authors.length > 0}
					<div>
						<span class="font-semibold"
							>Original Paper Written By:
							{$articleStore.authors.map((a) => a.name).join(', ')}</span
						>
					</div>
				{/if}
				<div>| Published on {formatDateString($articleStore.publishedAt)}</div>
				<div>| {$articleStore.readingTime} min read</div>
				{#if $articleStore.updatedAt}
					<div>| Updated on {formatDateString($articleStore.updatedAt)}</div>
				{/if}
				{#if $articleStore.lastUpdatedAt}
					<div>
						| Last Content Update: {formatDateString($articleStore.lastUpdatedAt)}
					</div>
				{/if}
			</div>
			<div class="mb-6 mt-1 flex flex-wrap items-center text-sm text-gray-400">
				<span class="font-semibold">Reviewed by: {$articleStore.professor.name}</span>
				{#if $articleStore.professor.professorBio}
					<span class="italic"> - {$articleStore.professor.professorBio}</span>
				{/if}
			</div>
		</div>

		<!-- Main Content Section (Article Body) -->
		<div class="mx-auto max-w-4xl leading-relaxed">
			<article class="prose prose-lg prose-invert max-w-none">
				<div
					class="prose"
					style="--tw-prose-body: inherit; --tw-prose-headings: inherit; --tw-prose-links: theme(colors.blue.400); --tw-prose-bold: inherit; --tw-prose-counters: inherit; --tw-prose-bullets: inherit; --tw-prose-hr: inherit; --tw-prose-quotes: inherit; --tw-prose-quote-borders: inherit; --tw-prose-captions: inherit; --tw-prose-kbd: inherit; --tw-prose-kbd-shadows: inherit; --tw-prose-code: inherit; --tw-prose-pre-code: inherit; --tw-prose-pre-bg: inherit; --tw-prose-th-borders: inherit; --tw-prose-td-borders: inherit; --tw-prose-invert-body: inherit; --tw-prose-invert-headings: inherit; --tw-prose-invert-links: theme(colors.blue.300); --tw-prose-invert-bold: inherit; --tw-prose-invert-counters: inherit; --tw-prose-invert-bullets: inherit; --tw-prose-invert-hr: inherit; --tw-prose-invert-quotes: inherit; --tw-prose-invert-quote-borders: inherit; --tw-prose-invert-captions: inherit; --tw-prose-invert-kbd: inherit; --tw-prose-invert-kbd-shadows: inherit; --tw-prose-invert-code: inherit; --tw-prose-invert-pre-code: inherit; --tw-prose-invert-pre-bg: inherit; --tw-prose-invert-th-borders: inherit; --tw-prose-invert-td-borders: inherit;"
				>
					{@html $articleStore.body
						.replaceAll(
							'<h1>',
							'<h1 class="mt-12 mb-6 text-3xl md:text-4xl font-extrabold leading-relaxed">'
						)
						.replaceAll(
							'<h2>',
							'<h2 class="mt-10 mb-4 text-2xl md:text-3xl font-bold leading-relaxed">'
						)
						.replaceAll(
							'<h3>',
							'<h3 class="mt-8 mb-3 text-xl md:text-2xl font-semibold leading-relaxed">'
						)
						.replaceAll(
							'<h4>',
							'<h4 class="mt-6 mb-2 text-lg md:text-xl font-semibold leading-relaxed">'
						)
						.replaceAll('<h5>', '<h5 class="mt-4 mb-2 text-md md:text-lg leading-relaxed">')
						.replaceAll('<h6>', '<h6 class="mt-2 mb-1 text-base md:text-md leading-relaxed">')
						.replaceAll('<p>', '<p class="my-2 text-base md:text-lg leading-relaxed">')
						.replaceAll('<ul>', '<ul class="list-disc list-inside space-y-2 mb-4">')
						.replaceAll('<ol>', '<ol class="list-decimal list-inside space-y-2 mb-4">')
						.replaceAll('<li>', '<li class="ml-6 mb-1">')
						.replaceAll(/<a\b/g, '<a class="text-blue-400 hover:text-blue-300 hover:underline"')}
				</div>
			</article>
		</div>

		<!-- Questions Section -->
		{#if $articleStore.questions && $articleStore.questions.length > 0}
			<div class="mx-auto mt-10 max-w-4xl">
				<h2 class="mb-6 text-2xl font-bold text-gray-200">Review Questions</h2>
				{#each $articleStore.questions as q, i}
					<div class="mb-6 rounded-lg border border-gray-700 bg-gray-700/30 p-4 shadow">
						<p class="mb-2 font-semibold text-gray-200">
							Question {i + 1}: {q.question}
						</p>
						<ul class="mb-2 list-disc space-y-1 pl-5 text-gray-300">
							{#each q.answers as ans}
								<li>{ans}</li>
							{/each}
						</ul>
						<p class="mt-1 text-sm font-medium text-green-400">
							Correct Answer: {q.correct_answer}
						</p>
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		<div class="mx-auto max-w-4xl p-8 text-center">
			<h1 class="mb-4 text-4xl font-bold">Article Not Found</h1>
			<p class="mb-8 text-lg text-gray-300">
				No article data is available for review. Please go back and fill out the form.
			</p>
			<Button
				onclick={() => {
					goto('/edit_toml');
				}}>Go to Editor</Button
			>
		</div>
	{/if}
</main>
