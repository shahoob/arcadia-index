<template>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
    <Card v-for="(album, index) in uploads" :key="index" class="p-card">
      <template #header> </template>
      <template #title>{{ album.title }}</template>
      <template #content>
        <p>{{ album.artist }}</p>
      </template>
    </Card>
  </div>
</template>

<script lang="ts">
import { ref } from 'vue'
import Card from 'primevue/card'

export default {
  components: { Card },
  setup() {
    const uploads = ref([
      { image: '/assets/album1.jpg', title: 'Album 1', artist: 'Artist 1' },
      { image: '/assets/album2.jpg', title: 'Album 2', artist: 'Artist 2' },
      { image: '/assets/album3.jpg', title: 'Album 3', artist: 'Artist 3' },
      { image: '/assets/album4.jpg', title: 'Album 4', artist: 'Artist 4' },
    ])

    return { uploads }
  },
}
</script>

<style scoped>
.album-cover {
  width: 100%;
  height: auto;
}
.p-card {
  background-color: #2c2c2c;
  color: white;
}
</style>
