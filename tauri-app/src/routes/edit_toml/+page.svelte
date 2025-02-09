<script lang="ts">
	import Header from './Header.svelte';
	import Sidebar from './Sidebar.svelte';
	import TomlForm from './TomlForm.svelte';
	let isSidebarOpen = $state(false);
</script>

<div class="min-h-screen w-full bg-background text-foreground">
	<Header bind:isSidebarOpen />

	<div class="flex h-[calc(100vh-4rem)]">
		<!-- Mobile sidebar -->
		<div
			class={[
				'fixed inset-y-0 left-0 z-50 w-72 transition-transform duration-200 lg:hidden',
				!isSidebarOpen && '-translate-x-full'
			]}
		>
			<Sidebar />
		</div>

		<!-- Desktop sidebar -->
		<div class="hidden pr-3 lg:block">
			<Sidebar />
		</div>

		<!-- Main content area -->
		<main class="flex-1 overflow-auto p-6">
			<TomlForm />
		</main>
	</div>
</div>

<!-- <main class="min-h-screen w-full bg-background p-4 text-foreground md:p-8">
</main> -->
