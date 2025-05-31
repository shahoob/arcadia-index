<template>
  <div id="app-container" v-if="isAppReady">
    <Toast />
    <div
      class="navbars-container"
      v-if="['/login', '/register'].indexOf(router.currentRoute.value.path) < 0"
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
import { Toast } from 'primevue'
import { useUserStore } from './stores/user'
import { getMe } from './services/api/userService'
import { ref } from 'vue'
import { useRoute } from 'vue-router'

// enable dark mode by default
document.documentElement.classList.add('dark-theme')

const isAppReady = ref(false)
const route = useRoute()
const router = useRouter()

router.isReady().then(async () => {
  const user = localStorage.getItem('user')
  if (user && route.path !== '/login' && route.path !== '/register') {
    // refresh user on page reload
    const profile = await getMe()
    localStorage.setItem('user', JSON.stringify(profile.user))
    const userStore = useUserStore()
    userStore.setUser(profile.user)
  } else if (!user) {
    router.push('/login')
  }
  isAppReady.value = true
})
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
