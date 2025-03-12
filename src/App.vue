<template>
  <div class="p-5">
    <h2>Welcome to PrimeVue in Arcadia Client</h2>
    <PrimeButton label="Fetch Data" @click="fetchData" />
    <p v-if="message">{{ message }}</p>
  </div>
</template>

<script lang="ts">
import { ref } from 'vue'
import Button from 'primevue/button'
import axios from 'axios'

export default {
  components: { PrimeButton: Button },
  setup() {
    const message = ref('')

    const fetchData = async () => {
      try {
        const res = await axios.get('/api/hello')
        message.value = res.data.message
      } catch (error) {
        console.error('Error fetching data:', error)
      }
    }

    return { message, fetchData }
  },
}
</script>
