<template>
  <div class="app-container">
    <!-- Navbar -->
    <div class="navbar-container">
      <AppNavbar />
    </div>

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
  </div>
</template>

<script lang="ts">
import AppNavbar from './components/AppNavbar.vue'
import LatestUploads from './components/LatestUploads.vue'
import AlbumOfTheMonth from './components/AlbumOfTheMonth.vue'

export default {
  components: {
    AppNavbar,
    LatestUploads,
    AlbumOfTheMonth,
  },
}
</script>

<style scoped>
/* Full-page container */
.app-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-height: 100vh;
  background-color: #1e1e1e; /* Dark theme */
  color: white;
}

/* Navbar Container */
.navbar-container {
  width: 80%;
  max-width: 1200px;
  margin-top: 20px;
}

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
