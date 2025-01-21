import { deserializeArticleFromToml, type Article } from '$lib/types';
import { fileURLToPath } from 'url';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import fs from 'fs/promises';
import path from 'path';
export const prerender = true;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const load: PageServerLoad = async ({ params }) => {
    const { slug } = params;
    const filePath = path.resolve(__dirname, '../../../../../../../src/articles', `${slug}.toml`);
    try {
        const articleToml = await fs.readFile(filePath, 'utf-8');
        // Deserialize the content into an Article object
        const article: Article | null = deserializeArticleFromToml(articleToml);
        if (!article) {
            throw error(404, 'Article not found');
        }
        return {
            article
        };
    } catch (err) {
        console.error('Error reading file:', err);
        throw error(404, 'Article not found');
    }
};
