<script lang="ts">
	import { MenuIcon, BellIcon } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Avatar, AvatarImage, AvatarFallback } from '$lib/components/ui/avatar';

	let { isSidebarOpen = $bindable() } = $props();
</script>

<header class="flex h-16 items-center justify-between border-b bg-card px-6">
	<div class="flex items-center gap-4">
		<Button
			variant="ghost"
			size="icon"
			class="lg:hidden"
			onclick={() => (isSidebarOpen = !isSidebarOpen)}
		>
			<MenuIcon class="size-5" />
		</Button>

		<h1 class="text-xl font-semibold">Dashboard</h1>
	</div>

	<div class="flex items-center gap-4">
		<Button variant="ghost" size="icon">
			<BellIcon class="size-5" />
		</Button>

		<Avatar>
			<AvatarImage src="https://api.dicebear.com/9.x/pixel-art/svg" alt="User" />
			<AvatarFallback>U</AvatarFallback>
		</Avatar>
	</div>
</header>
