<template>
  <a
    v-tooltip.top="title_group.platform ?? title_group.name"
    :href="`/title-group/${title_group.id}`"
  >
    <img :src="`/logos/platforms/${title_group.platform ?? title_group.content_type}.svg`" />
  </a>
</template>

<script setup lang="ts">
import { type TitleGroupLite } from '@/services/api/torrentService'

defineProps<{
  title_group: TitleGroupLite
}>()
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
