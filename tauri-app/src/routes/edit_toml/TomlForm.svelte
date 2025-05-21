<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { CalendarIcon } from 'lucide-svelte';
	import { Popover, PopoverContent, PopoverTrigger } from '$lib/components/ui/popover';
	import { Calendar } from '$lib/components/ui/calendar';
	import { formatDateForInput } from '$lib/utils';
	import {
		type DateValue,
		fromDate,
		getLocalTimeZone,
		parseAbsoluteToLocal
	} from '@internationalized/date';
	import { type Article, type Question, type Author, defaultArticle } from '$lib/types';
	import { articleStore } from '$lib/stores';
	import { goto } from '$app/navigation';

	let title: string = $state(defaultArticle.title);
	let description: string = $state(defaultArticle.description);
	let body: string = $state(defaultArticle.body);
	let category: string = $state(defaultArticle.category);
	let readingTime: number = $state(defaultArticle.readingTime);

	let createdAt: DateValue = $state(parseAbsoluteToLocal(defaultArticle.createdAt));
	let publishedAt: DateValue = $state(parseAbsoluteToLocal(defaultArticle.publishedAt));

	let updatedAt: DateValue | undefined = $state(
		defaultArticle.updatedAt && defaultArticle.updatedAt.trim() !== ''
			? parseAbsoluteToLocal(defaultArticle.updatedAt)
			: undefined
	);
	let lastUpdatedAt: DateValue | undefined = $state(
		defaultArticle.lastUpdatedAt && defaultArticle.lastUpdatedAt.trim() !== ''
			? parseAbsoluteToLocal(defaultArticle.lastUpdatedAt)
			: undefined
	);

	// Image section
	let imageUrl: string = $state(defaultArticle.image.url);
	let imageAlt: string = $state(defaultArticle.image.alt);
	let imageCaption: string = $state(defaultArticle.image.caption);

	// Authors section
	let authors: Author[] = $state(JSON.parse(JSON.stringify(defaultArticle.authors))); // Deep copy

	// Questions section
	let questions: Question[] = $state(JSON.parse(JSON.stringify(defaultArticle.questions))); // Deep copy

	// Professor section
	let professorName: string = $state(defaultArticle.professor.name);
	let professorBio: string = $state(defaultArticle.professor.professorBio);
	let professorSlug: string = $state(defaultArticle.professor.slug);

	const handleSubmit = () => {
		const articleData: Article = {
			title,
			description,
			body,
			category,
			readingTime,
			createdAt: createdAt.toAbsoluteString(),
			publishedAt: publishedAt.toAbsoluteString(),
			updatedAt: updatedAt ? updatedAt.toAbsoluteString() : null,
			lastUpdatedAt: lastUpdatedAt ? lastUpdatedAt.toAbsoluteString() : null,
			image: {
				url: imageUrl,
				alt: imageAlt,
				caption: imageCaption
			},
			authors: authors,
			professor: {
				name: professorName,
				professorBio: professorBio,
				slug: professorSlug
			},
			questions: questions
		};

		articleStore.set(articleData);
		goto('/edit_toml/review');
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
			<CardTitle>Edit Article Details</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="title">Title</Label>
				<Input id="title" bind:value={title} />
			</div>

			<div class="space-y-2">
				<Label for="description">Description</Label>
				<Textarea id="description" bind:value={description} />
			</div>

			<div class="space-y-2">
				<Label for="category">Category</Label>
				<Input id="category" bind:value={category} placeholder="e.g., research, chemistry" />
			</div>

			<div class="space-y-2">
				<Label for="body">Body (HTML)</Label>
				<Textarea id="body" bind:value={body} rows={10} />
			</div>

			<div class="space-y-2">
				<Label for="readingTime">Reading Time (minutes)</Label>
				<Input type="number" id="readingTime" bind:value={readingTime} />
			</div>

			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div class="space-y-2">
					<Label>Created At</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{formatDateForInput(createdAt)}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={createdAt} />
						</PopoverContent>
					</Popover>
				</div>

				<div class="space-y-2">
					<Label>Published At</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{formatDateForInput(publishedAt)}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={publishedAt} />
						</PopoverContent>
					</Popover>
				</div>

				<div class="space-y-2">
					<Label>Updated At (Optional)</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{updatedAt ? formatDateForInput(updatedAt) : 'Select date'}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={updatedAt} initialFocus />
						</PopoverContent>
					</Popover>
				</div>

				<div class="space-y-2">
					<Label>Last Content Update At (Optional)</Label>
					<Popover>
						<PopoverTrigger asChild>
							<Button variant="outline" class="w-full justify-start text-left">
								<CalendarIcon class="mr-2 size-4" />
								{lastUpdatedAt ? formatDateForInput(lastUpdatedAt) : 'Select date'}
							</Button>
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={lastUpdatedAt} initialFocus />
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
				<Input id="imageUrl" bind:value={imageUrl} />
			</div>

			<div class="space-y-2">
				<Label for="imageAlt">Alt Text</Label>
				<Input id="imageAlt" bind:value={imageAlt} />
			</div>

			<div class="space-y-2">
				<Label for="imageCaption">Caption</Label>
				<Input id="imageCaption" bind:value={imageCaption} />
			</div>
		</CardContent>
	</Card>

	<!-- Authors Section -->
	<Card>
		<CardHeader class="flex flex-row items-center justify-between">
			<CardTitle>Authors</CardTitle>
			<Button
				type="button"
				variant="outline"
				onclick={() => {
					authors = [
						...authors,
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
			{#each authors as author, index (index)}
				<div class="relative space-y-4 rounded-lg border p-4">
					{#if authors.length > 1}
						<Button
							type="button"
							variant="destructive"
							size="icon"
							class="absolute -right-2 -top-2 z-10"
							onclick={() => {
								authors = authors.filter((_, i) => i !== index);
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

	<!-- Professor Section -->
	<Card>
		<CardHeader>
			<CardTitle>Professor Details</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="professorName">Name</Label>
				<Input id="professorName" bind:value={professorName} />
			</div>

			<div class="space-y-2">
				<Label for="professorBio">Bio</Label>
				<Textarea id="professorBio" bind:value={professorBio} />
			</div>

			<div class="space-y-2">
				<Label for="professorSlug">Slug</Label>
				<Input id="professorSlug" bind:value={professorSlug} />
			</div>
		</CardContent>
	</Card>

	<!-- Question Section -->
	<Card>
		<CardHeader class="flex flex-row items-center justify-between">
			<CardTitle>Questions</CardTitle>
			<Button
				type="button"
				variant="outline"
				onclick={() => {
					questions = [
						...questions,
						{
							question: '',
							answers: ['', '', '', ''], // Default to 4 empty answers
							correct_answer: ''
						}
					];
				}}
			>
				Add Question
			</Button>
		</CardHeader>
		<CardContent class="space-y-6">
			{#each questions as question, index (index)}
				<div class="relative space-y-4 rounded-lg border p-4">
					{#if questions.length > 0}
						<!-- Allow removing even the last question -->
						<Button
							type="button"
							variant="destructive"
							size="icon"
							class="absolute -right-2 -top-2 z-10"
							onclick={() => {
								questions = questions.filter((_, i) => i !== index);
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
						<Label>Answers</Label>
						{#each question.answers as _, answerIndex (answerIndex)}
							<div class="flex items-center gap-2">
								<Input
									id={`answer-${index}-${answerIndex}`}
									bind:value={question.answers[answerIndex]}
									placeholder={`Answer ${answerIndex + 1}`}
								/>
								{#if question.answers.length > 1}
									<Button
										type="button"
										variant="ghost"
										size="icon"
										class="text-destructive hover:text-destructive"
										onclick={() => {
											question.answers = question.answers.filter((_, i) => i !== answerIndex);
											questions = questions; // Trigger reactivity
										}}
									>
										<span class="sr-only">Remove answer</span>X
									</Button>
								{/if}
							</div>
						{/each}
						<Button
							type="button"
							variant="outline"
							size="sm"
							onclick={() => {
								question.answers = [...question.answers, ''];
								questions = questions; // Trigger reactivity
							}}
						>
							Add Answer Option
						</Button>
					</div>

					<div class="space-y-2">
						<Label for={`correct_answer-${index}`}>Correct Answer</Label>
						<Input
							id={`correct_answer-${index}`}
							bind:value={question.correct_answer}
							placeholder="Copy the correct answer text here"
						/>
						<!-- Or use a select dropdown if answers are fixed -->
					</div>
				</div>
			{/each}
		</CardContent>
	</Card>

	<Button type="submit" class="!mb-12 w-full">Review & Save Changes</Button>
</form>
