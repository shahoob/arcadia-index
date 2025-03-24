<template>
  <div class="affiliated-artist">
    <Image :src="affiliated_artist.artist.pictures[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <div class="name">{{ affiliated_artist.artist.name }}</div>
    <div class="status">{{ affiliated_artist.status }}</div>
  </div>
</template>

<script lang="ts">
import { Image } from 'primevue'
export default {
  components: { Image },
  props: {
    affiliated_artist: {},
  },
}
</script>
<style scoped>
.affiliated-artist {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.status {
  font-size: 0.8em;
  color: var(--color-secondary);
}
</style>

<style>
.affiliated-artist img {
  height: 10em !important;
  border-radius: 7px;
}
</style>
