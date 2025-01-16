import { deserializeArticleFromToml, type Article } from '$lib/types';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { glob } from 'glob';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const articlesDir = path.resolve(__dirname, '..', '..', '..', 'articles');

export const prerender = true;

export const load: PageServerLoad = async ({ params }) => {
    const { slug } = params;
    const files = await glob('*.toml', { cwd: articlesDir });
    const matchingFile = files.find(file => path.parse(file).name === slug);

    if (!matchingFile) {
        throw error(404, 'Article not found');
    }

    const articleToml = await import(`/src/articles/${matchingFile}?raw`);
    const article: Article | null = deserializeArticleFromToml(articleToml.default);

    if (!article) {
        throw error(404, 'Article not found');
    }
    return {
        article
    };
};