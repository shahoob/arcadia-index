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
import * as helpers from './services/helpers'
import { createI18n } from 'vue-i18n'
import en from './i18n/en.json'
import fr from './i18n/fr.json'

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
const i18n = createI18n({
  locale: 'en',
  fallbackLocale: 'en', // Fallback if a translation is missing
  messages: {
    en,
    fr,
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
app.use(i18n)
app.directive('tooltip', Tooltip)

Object.keys(helpers).forEach((key) => {
  app.config.globalProperties[`$${key}`] = helpers[key as keyof typeof helpers]
})

app.mount('#app')
