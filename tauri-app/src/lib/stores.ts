// src/lib/stores.ts
import { writable } from 'svelte/store';
import type { Article } from '$lib/types';

// Your existing store (if any)
// For example, if you had this from previous context:
export const selected_answers = writable<string[]>([]);

// New store for the article being edited/displayed
export const articleStore = writable<Article | null>(null);

// Store for the GitHub token
export const githubTokenStore = writable<string | null>(null);