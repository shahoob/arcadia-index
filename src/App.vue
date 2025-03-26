<template>
  <div id="app-container">
    <!-- Navbar -->
    <div
      class="navbar-container"
      v-if="['/login', '/register'].indexOf($router.currentRoute.value.path) < 0"
    >
      <AppNavbar />
    </div>
    <div id="view-container">
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts">
import AppNavbar from './components/AppNavbar.vue'

export default {
  name: 'App',
  components: {
    AppNavbar,
  },
  setup() {
    // enable dark mode by default
    document.documentElement.classList.add('dark-theme')
  },
}
</script>

<style>
/* Navbar Container */
.navbar-container {
  width: 80%;
  margin-top: 10px;
  margin-bottom: 20px;
}
</style>
