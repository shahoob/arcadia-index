<template>
  <div class="name">{{ series.name }}</div>
</template>

<script setup lang="ts">
import type { Series } from '@/services/api/seriesService'

defineProps<{
  series: Series
}>()
</script>

<style scoped>
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
