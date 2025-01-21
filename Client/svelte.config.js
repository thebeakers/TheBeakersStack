import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { glob } from 'glob';
import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const articlesDir = path.resolve(__dirname, 'src', './articles');

const getArticleRoutes = () => {
	const files = glob.sync('*.toml', { cwd: articlesDir });
	const articleRoutes = files.map((file) => `/article/${path.parse(file).name}`);
	console.log('Found files:', articleRoutes);
	return ['*', ...articleRoutes];
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		files: {
			assets: 'static'
		},
		prerender: {
			entries: getArticleRoutes()
		}
	}
};

export default config;
