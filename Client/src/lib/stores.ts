import { writable } from 'svelte/store';

export const selected_answers = writable<string[]>([]);
