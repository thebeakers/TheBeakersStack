<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { CalendarIcon } from 'lucide-svelte';
	import { Popover, PopoverContent, PopoverTrigger } from '$lib/components/ui/popover';
	import { Calendar } from '$lib/components/ui/calendar';
	import { defaultArticle } from '$lib/types';
	import { formatDateForInput } from '$lib/utils';
	import { ArticleClass } from '$lib/classes.svelte';
	import { onMount } from 'svelte';

	function get_ArticleClass(): ArticleClass {
		return new ArticleClass(defaultArticle);
	}
	let article: ArticleClass = get_ArticleClass();
	onMount(() => {
		article = get_ArticleClass();
	});

	const handleSubmit = () => {
		const ArticleToml: ArticleClass = article as ArticleClass;
		console.log('Article TOML Data:', ArticleToml);
	};
</script>

<form
	class="space-y-6"
	onsubmit={(e) => {
		e.preventDefault();
		handleSubmit();
	}}
>
	<!-- Main Document Section -->
	<Card>
		<CardHeader>
			<CardTitle>Edit</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="title">Title</Label>
				<Input id="title" bind:value={article.title} />
			</div>

			<div class="space-y-2">
				<Label for="description">Description</Label>
				<Textarea id="description" bind:value={article.description} />
			</div>

			<div class="space-y-2">
				<Label for="body">Body</Label>
				<Textarea id="body" bind:value={article.body} rows={5} />
			</div>

			<div class="space-y-2">
				<Label for="readingTime">Reading Time (minutes)</Label>
				<Input type="number" id="readingTime" bind:value={article.readingTime} />
			</div>

			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div class="space-y-2">
					<Label>Created At</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{formatDateForInput(article.createdAt)}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={article.createdAt} />
						</PopoverContent>
					</Popover>
				</div>

				<div class="space-y-2">
					<Label>Published At</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{formatDateForInput(article.publishedAt)}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={article.publishedAt} />
						</PopoverContent>
					</Popover>
				</div>
			</div>
		</CardContent>
	</Card>

	<!-- Image Section -->
	<Card>
		<CardHeader>
			<CardTitle>Image Details</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="imageUrl">Image URL</Label>
				<Input id="imageUrl" bind:value={article.image.url} />
			</div>

			<div class="space-y-2">
				<Label for="imageAlt">Alt Text</Label>
				<Input id="imageAlt" bind:value={article.image.alt} />
			</div>

			<div class="space-y-2">
				<Label for="imageCaption">Caption</Label>
				<Input id="imageCaption" bind:value={article.image.caption} />
			</div>
		</CardContent>
	</Card>

	<!-- Authors Section -->
	<Card>
		<CardHeader class="flex flex-row items-center justify-between">
			<CardTitle>Authors</CardTitle>
			<Button
				variant="outline"
				onclick={() => {
					article.authors = [
						...article.authors,
						{
							name: '',
							authorBio: '',
							slug: ''
						}
					];
				}}
			>
				Add Author
			</Button>
		</CardHeader>
		<CardContent class="space-y-6">
			{#each article.authors as author, index (index)}
				<div class="relative space-y-4 rounded-lg border p-4">
					{#if article.authors.length > 1}
						<Button
							variant="destructive"
							size="icon"
							class="absolute -right-2 -top-2"
							onclick={() => {
								article.authors = article.authors.filter((_, i) => i !== index);
							}}
						>
							<span class="sr-only">Remove author</span>
							X
						</Button>
					{/if}

					<div class="space-y-2">
						<Label for={`author-name-${index}`}>Name</Label>
						<Input id={`author-name-${index}`} bind:value={author.name} />
					</div>

					<div class="space-y-2">
						<Label for={`author-bio-${index}`}>Bio</Label>
						<Textarea id={`author-bio-${index}`} bind:value={author.authorBio} />
					</div>

					<div class="space-y-2">
						<Label for={`author-slug-${index}`}>Slug</Label>
						<Input id={`author-slug-${index}`} bind:value={author.slug} />
					</div>
				</div>
			{/each}
		</CardContent>
	</Card>

	<!-- Question Section -->
	<Card>
		<CardHeader class="flex flex-row items-center justify-between">
			<CardTitle>Questions</CardTitle>
			<Button
				variant="outline"
				onclick={() => {
					article.questions = [
						...article.questions,
						{
							question: '',
							answers: ['', '', '', ''],
							correct_answer: ''
						}
					];
				}}
			>
				Add Question
			</Button>
		</CardHeader>
		<CardContent class="space-y-6">
			{#each article.questions as question, index (index)}
				<div class="relative space-y-4 rounded-lg border p-4">
					{#if article.questions.length > 1}
						<Button
							variant="destructive"
							size="icon"
							class="absolute -right-2 -top-2"
							onclick={() => {
								article.questions = article.questions.filter((_, i) => i !== index);
							}}
						>
							<span class="sr-only">Remove Question</span>
							X
						</Button>
					{/if}

					<div class="space-y-2">
						<Label for={`question-${index}`}>Question {index + 1}</Label>
						<Textarea id={`question-${index}`} bind:value={question.question} />
					</div>

					<div class="space-y-2">
						<Label for={`answers-${index}`}>Answers</Label>
						{#each question.answers as answer, answerIndex (answerIndex)}
							<div class="space-y-2">
								<Label for={`answer-${index}-${answerIndex}`}>Answer {answerIndex + 1}</Label>
								<Input
									id={`answer-${index}-${answerIndex}`}
									bind:value={question.answers[answerIndex]}
								/>
							</div>
						{/each}
					</div>

					<div class="space-y-2">
						<Label for={`correct_answer-${index}`}>Correct Answer</Label>
						<Input id={`correct_answer-${index}`} bind:value={question.correct_answer} />
					</div>
				</div>
			{/each}
		</CardContent>
	</Card>
	<!-- Professor Section -->
	<Card>
		<CardHeader>
			<CardTitle>Professor Details</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="professorName">Name</Label>
				<Input id="professorName" bind:value={article.professor.name} />
			</div>

			<div class="space-y-2">
				<Label for="professorBio">Bio</Label>
				<Textarea id="professorBio" bind:value={article.professor.professorBio} />
			</div>

			<div class="space-y-2">
				<Label for="professorSlug">Slug</Label>
				<Input id="professorSlug" bind:value={article.professor.slug} />
			</div>
		</CardContent>
	</Card>

	<Button type="submit" class="w-full">Save Changes</Button>
</form>
