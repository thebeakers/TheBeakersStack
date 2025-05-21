<script lang="ts">
	import { articleStore, githubTokenStore } from '$lib/stores';
	import type { Article } from '$lib/types'; // This Article type is from your Svelte $lib/types
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	// TOML stringify is no longer needed here

	let article: Article | null = null;
	let ghToken: string | null = null; // Used for UI logic (e.g., disabling button)
	let isUploading = $state(false);
	let uploadMessage = $state('');
	let uploadError = $state(false);

	const unsubscribeArticle = articleStore.subscribe((value) => {
		article = value;
	});
	const unsubscribeToken = githubTokenStore.subscribe((value) => {
		ghToken = value;
	});

	onDestroy(() => {
		unsubscribeArticle();
		unsubscribeToken();
	});

	function formatDateString(dateString: string | null | undefined): string {
		if (!dateString || dateString.trim() === '') {
			return 'Not set';
		}
		try {
			// Assuming dateString is a valid ISO string or parsable by Date constructor
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
			.replace(/\s+/g, '-') // Replace spaces with -
			.replace(/[^\w-]+/g, '') // Remove all non-word chars
			.replace(/--+/g, '-') // Replace multiple - with single -
			.replace(/^-+/, '') // Trim - from start of text
			.replace(/-+$/, ''); // Trim - from end of text
	}

	const handleUploadToGitHub = async () => {
		if (!article) {
			uploadMessage = 'Error: No article data to upload.';
			uploadError = true;
			return;
		}
		// Client-side check for token to manage UI, Rust side will do the definitive check
		if (!ghToken) {
			uploadMessage = 'Error: GitHub token is missing. Please log in again.';
			uploadError = true;
			return;
		}

		isUploading = true;
		uploadMessage = 'Preparing to upload...';
		uploadError = false;

		try {
			const fileName = `${slugify(article.title)}.toml`;

			uploadMessage = `Uploading article '${article.title}' to GitHub as ${fileName}...`;
			console.log(`Preparing to upload: ${fileName}`);
			// For debugging, ensure the article object structure matches Rust's substuff::Article
			console.log('Article data being sent to Rust:', JSON.parse(JSON.stringify(article)));

			// The 'article' object passed here must be serializable by serde
			// to match the `substuff::Article` struct in Rust.
			// Ensure field names and types are compatible (e.g., authorBio vs author_bio handled by serde rename).
			const result = await invoke<string>('upload_article_to_github', {
				article: article,
				fileName: fileName
			});

			uploadMessage = `Success: ${result}`;
			uploadError = false;
		} catch (error: any) {
			console.error('Error uploading to GitHub:', error);
			const errorMessage = typeof error === 'string' ? error : error?.message || JSON.stringify(error);
			uploadMessage = `Failed to upload: ${errorMessage}`;
			uploadError = true;
		} finally {
			isUploading = false;
		}
	};
</script>

<main class="min-h-screen bg-gray-800 p-6 font-serif text-gray-100">
	{#if article}
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
					disabled={isUploading || !ghToken}
					title={!ghToken ? 'Please log in with GitHub first' : 'Upload to GitHub repository'}
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
			{#if !ghToken && !isUploading}
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

		<!-- Header Section (Article Display) -->
		<div class="mx-auto max-w-4xl">
			<div class="mb-4">
				<img
					src={article.image.url}
					alt={article.image.alt}
					class="h-auto w-full rounded-md object-cover"
				/>
				<p class="mt-2 text-center text-sm italic text-gray-400">
					{article.image.caption}
				</p>
			</div>
			<h1 class="mb-2 text-4xl font-bold">{article.title}</h1>
			<p class="mb-4 text-lg text-gray-300">{article.description}</p>
			<div class="mb-2 text-sm text-gray-400">
				<span class="font-semibold">Category:</span>
				{article.category ?? 'N/A'} <!-- Display category if available -->
			</div>
			<div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-gray-400">
				{#if article.authors && article.authors.length > 0}
					<div>
						<span class="font-semibold"
							>Original Paper Written By:
							{article.authors.map((a) => a.name).join(', ')}</span
						>
					</div>
				{/if}
				<div>| Published on {formatDateString(article.publishedAt)}</div>
				<div>| {article.readingTime} min read</div>
				{#if article.updatedAt} <!-- Check for existence, not just empty string -->
					<div>| Updated on {formatDateString(article.updatedAt)}</div>
				{/if}
				{#if article.lastUpdatedAt} <!-- Check for existence -->
					<div>
						| Last Content Update: {formatDateString(article.lastUpdatedAt)}
					</div>
				{/if}
			</div>
			<div class="mb-6 mt-1 flex flex-wrap items-center text-sm text-gray-400">
				<span class="font-semibold">Reviewed by: {article.professor.name}</span>
				{#if article.professor.professorBio}
					<span class="italic"> - {article.professor.professorBio}</span>
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
					{@html article.body
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
		{#if article.questions && article.questions.length > 0}
			<div class="mx-auto mt-10 max-w-4xl">
				<h2 class="mb-6 text-2xl font-bold text-gray-200">Review Questions</h2>
				{#each article.questions as q, i}
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