<script lang="ts">
    import type { Article } from '$lib/types';

    export let article: Article = {
        title: "Default Title",
        description: "Default Description",
        body: "<p>Default Body</p>",
        image: {
          url: "https://placekitten.com/640/360",
          alt: "Default Image",
          caption: "Default Caption",
        },
        authors: [
          {
            name: "Default Author",
            authorBio: "Default Author Bio",
            slug: "default-author",
          },
        ],
        
        professor: {
          name: "Default Professor",
          professorBio: "Default Professor Bio",
          slug: "default-professor",
        },
        createdAt: new Date(new Date().getTime() - 7 * 24 * 60 * 60 * 1000).toISOString(),
        publishedAt: new Date(new Date().getTime() - 24 * 60 * 60 * 1000).toISOString(),
        updatedAt: null,
        lastUpdatedAt: null,
        readingTime: 5,
      };


    

</script>
<main class="p-6 bg-gray-100 text-gray-900 font-serif">
    <!-- Header Section -->
    <div class="max-w-4xl mx-auto">
      <!-- Article Image -->
      <div class="mb-4">
        <img src={article.image.url} alt={article.image.alt} class="w-full h-auto rounded-md" />
        <p class="mt-2 text-sm text-gray-600 italic text-center">{article.image.caption}</p>
      </div>
  
      <!-- Title and Description -->
      <h1 class="text-4xl font-bold mb-2">{article.title}</h1>
      <p class="text-lg text-gray-700 mb-4">{article.description}</p>
  
      <!-- Author and Metadata -->
      <div class="flex flex-wrap items-center gap-4 text-sm text-gray-600 mb-6">
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
      <article class="prose prose-lg max-w-none">
        {@html article.body}
      </article>
    </div>
  
  </main>
  
  <style>
    :global(body) {
      font-family: 'Georgia', serif;
    }
  </style>