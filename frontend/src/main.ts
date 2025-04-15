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
import { definePreset } from '@primeuix/themes'

const app = createApp(App)

const CustomThemePreset = definePreset(Aura, {
  semantic: {
    primary: {
      200: '#BD3333',
      300: '#5fe1ab', //primary
      400: '#876ED0', //secondary
      500: '#BD3333',
      600: '#BD3333',
      700: '#BD3333',
      800: '#BD3333',
      900: '#BD3333',
    },
  },
})

app.use(createPinia())
app.use(PrimeVue, {
  theme: {
    preset: CustomThemePreset,
    options: {
      darkModeSelector: '.dark-theme',
    },
  },
})
app.use(router)
app.directive('tooltip', Tooltip)
app.mount('#app')
