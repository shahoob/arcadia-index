import { createApp } from 'vue'
import App from './App.vue'
import PrimeVue from 'primevue/config'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'
import Aura from '@primeuix/themes/aura'
import { createPinia } from 'pinia'
import router from './router'
import 'primeicons/primeicons.css'
import './assets/main.css'
import Tooltip from 'primevue/tooltip'

const app = createApp(App)

app.use(createPinia())
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.my-app-dark',
    },
  },
})
app.use(router)
app.directive('tooltip', Tooltip)
app.mount('#app')
