import { invoke } from '@tauri-apps/api/tauri';

async function getArticleFromRust(filePath: string) {
    try {
        // Call the Rust command and pass the file path as an argument
        const article = await invoke('get_article', { filePath });
        console.log('Article from Rust:', article);
        return article;
    } catch (error) {
        console.error('Error invoking get_article command:', error);
        throw error;
    }
}

export default getArticleFromRust;
