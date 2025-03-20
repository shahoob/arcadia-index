<template>
  <div class="app-container">
    <!-- Navbar -->
    <div class="navbar-container">
      <AppNavbar />
    </div>
    <div id="app"><router-view></router-view></div>
  </div>
</template>

<script lang="ts">
import AppNavbar from './components/AppNavbar.vue'

export default {
  name: 'App',
  components: {
    AppNavbar,
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
</style>
