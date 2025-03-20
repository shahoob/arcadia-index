<template>
  <div id="app-container">
    <!-- Navbar -->
    <div class="navbar-container">
      <AppNavbar />
    </div>
    <router-view></router-view>
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

<style>
/* Navbar Container */
.navbar-container {
  width: 80%;
  margin-top: 10px;
}
</style>
