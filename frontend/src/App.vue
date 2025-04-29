<template>
  <div id="app-container">
    <div
      class="navbars-container"
      v-if="['/login', '/register'].indexOf($router.currentRoute.value.path) < 0"
    >
      <TopBar />
      <MenuBar class="menu-bar" />
      <SearchBars class="search-bars" />
    </div>
    <div id="view-container">
      <router-view></router-view>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import MenuBar from './components/MenuBar.vue'
import TopBar from './components/TopBar.vue'
import SearchBars from './components/SearchBars.vue'
import { useUserStore } from './stores/user'

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
.navbars-container {
  width: 100%;
}
.menu-bar {
  margin-top: 10px;
}
.search-bars {
  margin-top: 10px;
  margin-bottom: 20px;
}
</style>
