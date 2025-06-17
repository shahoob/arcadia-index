<template>
  <div id="app-container" v-if="isAppReady">
    <Toast />
    <div class="navbars-container" v-if="isProtectedRoute">
      <TopBar />
      <MenuBar class="menu-bar" />
      <SearchBars class="search-bars" />
    </div>
    <div id="view-container">
      <router-view></router-view>
    </div>
    <FooterBar />
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
import { computed } from 'vue'
import FooterBar from './components/FooterBar.vue'

// enable dark mode by default
document.documentElement.classList.add('dark-theme')

const isAppReady = ref(false)
const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME

const isProtectedRoute = computed(() => {
  return ['/login', '/register', '/apply'].indexOf(route.path) < 0
})

router.beforeEach((to, from, next) => {
  if (to.meta.dynamicDocumentTitle) {
    /*
      The View for this route handles it's own
      document title because it's dynamic.
    */
    return next()
  }

  document.title = `${to.meta.documentTitle || to.name} - ${siteName}`

  return next()
})

router.isReady().then(async () => {
  const token = localStorage.getItem('token')

  if (isProtectedRoute.value) {
    if (token) {
      try {
        // refresh user on page reload or fetch user after registration
        const profile = await getMe()
        localStorage.setItem('user', JSON.stringify(profile.user))
        const userStore = useUserStore()
        userStore.setUser(profile.user)
      } catch {
        // Token is invalid, redirect to login
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        router.push('/login')
      }
    } else {
      router.push('/login')
    }
  }
  isAppReady.value = true
})
</script>

<style>
.navbars-container {
  width: 100vw;
}
.menu-bar {
  margin-top: 10px;
}
.search-bars {
  margin-top: 10px;
  margin-bottom: 20px;
}
#footer {
  margin-top: auto;
  width: 100vw;
}
</style>
