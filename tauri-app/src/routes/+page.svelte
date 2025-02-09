<script lang="ts">
	import { goto } from '$app/navigation';

	// import Login from './LoginLocal.svelte';
	import Login from './LoginGit.svelte';
</script>

<main
	class="flex min-h-screen w-full items-center justify-center bg-background p-4 text-foreground"
>
	<Login />
	<button
		on:click={() => {
			goto('/edit_toml');
		}}>AAAA</button
	>
</main>
