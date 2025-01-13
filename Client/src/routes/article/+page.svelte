<script lang="ts">
    import { deserializeArticleFromToml, type Article } from '$lib/types';
    import articleToml from '../../../static/article.toml?raw';

   
    let article: Article | null = deserializeArticleFromToml(articleToml);



</script>
<main class="p-6 bg-gray-800 text-gray-100 font-serif min-h-screen">

    {#if article}
    <!-- Header Section -->
    <div class="max-w-4xl mx-auto">
      <!-- Article Image -->
      <div class="mb-4">
        <img src={article.image.url} alt={article.image.alt} class="w-full h-auto rounded-md" />
        <p class="mt-2 text-sm text-gray-400 italic text-center">{article.image.caption}</p>
      </div>
  
      <!-- Title and Description -->
      <h1 class="text-4xl font-bold mb-2">{article.title}</h1>
      <p class="text-lg text-gray-300 mb-4">{article.description}</p>
  
      <!-- Author and Metadata -->
      <div class="flex flex-wrap items-center gap-4 text-sm text-gray-400 mb-6">
        <div>
          <span class="font-semibold">{article.authors[0].name}</span>
          <span class="italic">- {article.authors[0].authorBio}</span>
        </div>
        <div>| Published on {new Date(article.publishedAt).toLocaleDateString()}</div>
        <div>| {article.readingTime} min read</div>
      </div>
    </div>
  
    <!-- Main Content Section -->
    <div class="max-w-4xl mx-auto leading-relaxed">
      <article class="prose prose-lg max-w-none prose-invert">
        {@html article.body}
      </article>
    </div>
    {:else}
    <p>Loading...</p>
    {/if}
    
    

  
  </main>
  
  <style>
    :global(body) {
      font-family: 'Georgia', serif;
    }
  </style>