<script lang="ts">
	import { z } from 'zod';
	import { LockIcon, BookOpenIcon } from 'lucide-svelte';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { goto } from '$app/navigation';

	// Form validation schema
	const loginSchema = z.object({
		username: z.string().min(1, 'Username is required'),
		password: z
			.string()
			.min(1, 'Password is required')
			.min(8, 'Password must be at least 8 characters long')
	});

	// Form state
	let username: string = $state('');
	let password: string = $state('');
	let errors: { username?: string; password?: string } = $state({});

	// Handle form submission
	const onsubmit = (e: Event) => {
		e.preventDefault();
		errors = {};

		try {
			loginSchema.parse({ username, password });
			// Here you would typically make an API call to authenticate
			console.log('Form submitted:', { username, password });
			goto('./edit_toml');
		} catch (err) {
			if (err instanceof z.ZodError) {
				err.errors.forEach((error) => {
					const field = error.path[0] as 'username' | 'password';
					errors[field] = error.message;
				});
			}
		}
	};
</script>

<div class="w-full max-w-md">
	<Card>
		<CardHeader>
			<div class="mb-2 flex items-center gap-2">
				<BookOpenIcon class="size-6 text-primary" />
				<CardTitle>Professor Article Editor</CardTitle>
			</div>
			<CardDescription>Please sign in to access your articles and editing tools</CardDescription>
		</CardHeader>
		<CardContent>
			<form {onsubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-2">
					<Label for="username">Username</Label>
					<Input
						id="username"
						type="text"
						bind:value={username}
						placeholder="Enter your username"
						class={errors.username ? 'border-destructive' : ''}
					/>
					{#if errors.username}
						<p class="text-sm text-destructive">{errors.username}</p>
					{/if}
				</div>

				<div class="flex flex-col gap-2">
					<Label for="password">Password</Label>
					<Input
						id="password"
						type="password"
						bind:value={password}
						placeholder="Enter your password"
						class={errors.password ? 'border-destructive' : ''}
					/>
					{#if errors.password}
						<p class="text-sm text-destructive">{errors.password}</p>
					{/if}
				</div>

				<Button type="submit" class="mt-2 w-full">
					<LockIcon class="mr-2 size-4" />
					Sign In
				</Button>
			</form>
		</CardContent>
	</Card>
</div>
