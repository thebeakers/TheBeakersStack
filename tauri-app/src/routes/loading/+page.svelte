<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	onMount(() => {
		invoke('sync_git_in_cache').catch((e) => {
			console.error('Error during auth:', e);
		});
	});
</script>
