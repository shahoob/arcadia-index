<template>
  <div class="name">{{ artist.name }}</div>
</template>
<script setup lang="ts">
import { type Artist } from '@/services/api/artistService'

defineProps<{
  artist: Artist
}>()
</script>
<style scoped>
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
