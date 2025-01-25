<script lang="ts">
	import type { Article } from '$lib/types';
	import type { PageData } from './$types';

	export let data: PageData;
	let article: Article | null = data.article;
</script>

<main class="min-h-screen bg-gray-800 p-6 font-serif text-gray-100">
	{#if article}
		<!-- Header Section -->
		<div class="mx-auto max-w-4xl">
			<!-- Article Image -->
			<div class="mb-4">
				<img src={article.image.url} alt={article.image.alt} class="h-auto w-full rounded-md" />
				<p class="mt-2 text-center text-sm italic text-gray-400">{article.image.caption}</p>
			</div>

			<!-- Title and Description -->
			<h1 class="mb-2 text-4xl font-bold">{article.title}</h1>
			<p class="mb-4 text-lg text-gray-300">{article.description}</p>

			<!-- Author and Metadata -->
			<div class="flex flex-wrap items-center gap-4 text-sm text-gray-400">
				<div>
					<span class="font-semibold">Orginal Paper Written By: {article.authors[0].name}</span>
					<span class="italic"> - {article.authors[0].authorBio}</span>
				</div>

				<div>| Published on {new Date(article.publishedAt).toLocaleDateString()}</div>
				<div>| {article.readingTime} min read</div>
			</div>
			<div class="mb-6 flex flex-wrap items-center text-sm text-gray-400">
				<span class="font-semibold">Reviewed by: {article.professor.name}</span>
				<span class="italic"> - {article.professor.professorBio}</span>
			</div>
		</div>

		<!-- Main Content Section -->
		<div class="mx-auto max-w-4xl leading-relaxed">
			<article class="prose prose-lg prose-invert max-w-none">
				{@html article.body}
			</article>
		</div>
	{:else}
		<div class="mx-auto max-w-4xl p-8 text-center">
			<h1 class="mb-4 text-4xl font-bold">Woops!</h1>
			<p class="mb-8 text-lg text-gray-300">
				We seem to have misplaced this article. It might be a temporary glitch, or the page might
				have been removed.
			</p>
			<p class="mb-4 text-gray-400">Please try one of the following:</p>
			<ul class="mb-8 text-gray-400">
				<li class="mb-2">
					<a href="/" class="text-blue-500 hover:underline">Go back to the homepage</a>
				</li>
				<li>
					<button class="text-blue-500 hover:underline" on:click={() => window.history.back()}
						>Go back to the previous page</button
					>
				</li>
			</ul>
			<p class="text-sm text-gray-500">If the problem persists, please contact support.</p>
		</div>
	{/if}
</main>
