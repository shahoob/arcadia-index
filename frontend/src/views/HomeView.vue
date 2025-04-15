<template>
  <main>
    <!-- Main Layout -->
    <div class="main-layout">
      <!-- Latest Uploads -->
      <main class="content">
        <h2 class="text-2xl font-semibold mb-4">Latest Uploads</h2>
        <LatestUploads />
      </main>

      <!-- Sidebar -->
      <aside class="sidebar">
        <AlbumOfTheMonth />
        <BlogPosts />
      </aside>
    </div>
  </main>
</template>

<script setup lang="ts">
import AlbumOfTheMonth from '@/components/AlbumOfTheMonth.vue'
import BlogPosts from '@/components/BlogPosts.vue'
import LatestUploads from '@/components/LatestUploads.vue'
</script>

<style>
/* Main Content Layout */
.main-layout {
  display: flex;
  flex-direction: row;
  width: 80%;
  max-width: 1200px;
  margin-top: 20px;
  gap: 20px;
}

/* Latest Uploads */
.content {
  flex: 7;
  display: flex;
  flex-direction: column;
}

/* Sidebar */
.sidebar {
  flex: 3;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
