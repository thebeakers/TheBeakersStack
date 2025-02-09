import { type Article } from '$lib/types';
import { deserializeArticleFromToml } from "$lib/utils"
import { fileURLToPath } from 'url';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import fs from 'fs/promises';
export const prerender = true;

const __filename = fileURLToPath(import.meta.url);

export const load: PageServerLoad = async ({ params }) => {
    let { slug } = params
    let filePath = getRelativePathToSrc(slug);
    try {
        const articleToml = await fs.readFile(filePath, 'utf-8');
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
function getRelativePathToSrc(slug: string) {
    let currentDir = process.cwd(); // Get the current working directory
    console.log(currentDir)
    return `${currentDir}/src/articles/${slug}.toml`
}
