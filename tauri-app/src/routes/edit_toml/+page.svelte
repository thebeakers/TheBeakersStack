<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
	import TomlForm from "./TomlForm.svelte";
</script>

<main class="min-h-screen w-full bg-background text-foreground p-4 md:p-8">
	<Card class="max-w-4xl mx-auto">
		<CardHeader>
			<CardTitle>TOML Document Editor</CardTitle>
		</CardHeader>
		<CardContent>
			<TomlForm />
		</CardContent>
	</Card>
</main>
