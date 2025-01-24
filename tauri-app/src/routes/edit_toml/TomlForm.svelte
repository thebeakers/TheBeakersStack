<script lang="ts">
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import { Textarea } from "$lib/components/ui/textarea";
	import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
	import { CalendarIcon } from "lucide-svelte";
	import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
	import { Calendar } from "$lib/components/ui/calendar";
    function formatDateForInput(date: Date): string {
	return new Intl.DateTimeFormat("en-US", {
		year: "numeric",
		month: "long",
		day: "numeric",
		hour: "2-digit",
		minute: "2-digit",
	}).format(date);
}


	// Main document data
	let title: string = $state("Default Title");
	let description: string = $state("Default Description");
	let body: string = $state("<p>Default Body</p>");
	let readingTime: number = $state(10);
	let createdAt: Date = $state(new Date("2024-02-22T16:40:18.000Z"));
	let publishedAt: Date = $state(new Date("2024-02-29T16:40:18.000Z"));
	let updatedAt: Date = $state(new Date());
	let lastUpdatedAt: Date = $state(new Date());

	// Image section
	let imageUrl: string = $state("https://placehold.co/600x400");
	let imageAlt: string = $state("Default Image");
	let imageCaption: string = $state("Default Caption");

	// Authors section
	interface Author {
		name: string;
		authorBio: string;
		slug: string;
	}
	let authors: Author[] = $state([
		{
			name: "Default Author",
			authorBio: "Default Author Bio",
			slug: "default-author",
		},
	]);

	// Professor section
	let professorName: string = $state("Default Professor");
	let professorBio: string = $state("Default Professor Bio");
	let professorSlug: string = $state("default-professor");

	const handleSubmit = () => {
		const tomlData = {
			title,
			description,
			body,
			createdAt: createdAt.toISOString(),
			publishedAt: publishedAt.toISOString(),
			updatedAt: updatedAt.toISOString(),
			lastUpdatedAt: lastUpdatedAt.toISOString(),
			readingTime,
			image: {
				url: imageUrl,
				alt: imageAlt,
				caption: imageCaption,
			},
			authors: authors,
			professor: {
				name: professorName,
				professorBio: professorBio,
				slug: professorSlug,
			},
		};
		
		console.log("TOML Data:", tomlData);
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
			<CardTitle>Main Document</CardTitle>
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
				<Label for="body">Body</Label>
				<Textarea id="body" bind:value={body} rows={5} />
			</div>

			<div class="space-y-2">
				<Label for="readingTime">Reading Time (minutes)</Label>
				<Input type="number" id="readingTime" bind:value={readingTime} />
			</div>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<div class="space-y-2">
					<Label>Created At</Label>
					<Popover>
						<PopoverTrigger>
							{#snippet child({ props: triggerProps })}
								<Button variant="outline" class="w-full justify-start text-left" {...triggerProps}>
									<CalendarIcon class="mr-2 size-4" />
									{formatDateForInput(createdAt)}
								</Button>
							{/snippet}
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={createdAt} />
						</PopoverContent>
					</Popover>
				</div>

				<div class="space-y-2">
					<Label>Published At</Label>
					<Popover>
						<PopoverTrigger>
							{#snippet child({ props: triggerProps })}
								<Button variant="outline" class="w-full justify-start text-left" {...triggerProps}>
									<CalendarIcon class="mr-2 size-4" />
									{formatDateForInput(publishedAt)}
								</Button>
							{/snippet}
						</PopoverTrigger>
						<PopoverContent class="w-auto p-0">
							<Calendar bind:value={publishedAt} />
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
				variant="outline"
				onclick={() => {
					authors = [
						...authors,
						{
							name: "",
							authorBio: "",
							slug: "",
						},
					];
				}}
			>
				Add Author
			</Button>
		</CardHeader>
		<CardContent class="space-y-6">
			{#each authors as author, index (index)}
				<div class="space-y-4 border rounded-lg p-4 relative">
					{#if authors.length > 1}
						<Button
							variant="destructive"
							size="icon"
							class="absolute -top-2 -right-2"
							onclick={() => {
								authors = authors.filter((_, i) => i !== index);
							}}
						>
							<span class="sr-only">Remove author</span>
							×
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

	<Button type="submit" class="w-full">Save Changes</Button>
</form>
