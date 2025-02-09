<script lang="ts">
	import { FilesIcon, FolderIcon } from 'lucide-svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { Separator } from '$lib/components/ui/separator';

	// Sample files data
	const files = [
		{ name: 'Documents', icon: FolderIcon, items: ['Report.pdf', 'Invoice.doc'] },
		{ name: 'Images', icon: FolderIcon, items: ['photo1.jpg', 'photo2.jpg'] },
		{ name: 'Projects', icon: FolderIcon, items: ['Project A', 'Project B'] }
	];
</script>

<aside class="h-full border-r bg-card">
	<ScrollArea class="h-full">
		<div class="space-y-4 px-2 py-2">
			<div class="flex items-center gap-2 px-4 py-2">
				<FilesIcon class="size-5" />
				<span class="font-semibold">Files</span>
			</div>

			<Separator />

			{#each files as folder}
				<div class="text-md">
					<div
						class="flex cursor-pointer items-center gap-2 rounded-md px-4 font-medium hover:bg-accent"
					>
						<svelte:component this={folder.icon} class="size-4" />
						{folder.name}
					</div>

					<div class="ml-6">
						{#each folder.items as item}
							<!-- svelte-ignore a11y_no_static_element_interactions -->
							<!-- svelte-ignore a11y_click_events_have_key_events -->
							<div
								class="cursor-pointer rounded-md px-4 py-2 text-sm text-muted-foreground hover:bg-accent hover:text-foreground"
								on:click={() => {
									console.log(folder.items);
								}}
							>
								{item}
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</ScrollArea>
</aside>
