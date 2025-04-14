<template>
  <div id="app-container">
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
import { useRouter } from 'vue-router'
import AppNavbar from './components/AppNavbar.vue'
import { useUserStore } from './stores/user'

export default {
  name: 'App',
  components: {
    AppNavbar,
  },
}
</script>
<script setup lang="ts">
// enable dark mode by default
document.documentElement.classList.add('dark-theme')

const user = localStorage.getItem('user')
if (user) {
  const userStore = useUserStore()
  userStore.setUser(JSON.parse(user))
} else {
  const router = useRouter()
  router.push('/login')
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
