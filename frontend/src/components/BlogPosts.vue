<template>
  <Card class="p-card">
    <template #header>
      <h3>Latest Blog Posts</h3>
    </template>
    <template #content>
      <ul>
        <li v-for="(post, index) in blogPosts" :key="index">{{ post }}</li>
      </ul>
    </template>
  </Card>
</template>

<script lang="ts">
import { ref } from 'vue'
import Card from 'primevue/card'

export default {
  components: { Card },
  setup() {
    const blogPosts = ref(['Server maintenance [Completed]'])
    return { blogPosts }
  },
}
</script>

<style scoped>
.p-card {
  background-color: #2c2c2c;
  color: white;
}
</style>
