import { mdsvex } from 'mdsvex';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from '@sveltejs/adapter-cloudflare';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [vitePreprocess(), mdsvex()],

	kit: {
        adapter: adapter(),
        paths: {
            base: process.env.NODE_ENV === 'production' ? '/sveltekit-github-pages' : '',
        }
    },

	extensions: ['.svelte', '.svx']
};

export default config;
