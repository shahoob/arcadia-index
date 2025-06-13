<template>
  <div class="title">
    <span v-if="title_group.series?.id">
      <RouterLink class="prefix" :to="`/series/${title_group.series.id}`">{{ title_group.series.name }} </RouterLink>
      -
    </span>
    <span v-if="title_group.platform">
      <RouterLink class="prefix" :to="`/platform?name=${title_group.platform}`">
        {{ title_group.platform }}
      </RouterLink>
      -
    </span>
    <span class="title-group-name">{{ title_group.name }}</span>
    <span class="year">({{ title_group.original_release_date.substring(0, 4) }})</span>
  </div>
</template>
<script setup lang="ts">
import type { TitleGroupAndAssociatedData } from '@/services/api/torrentService'

defineProps<{
  title_group: TitleGroupAndAssociatedData
}>()
</script>
<style scoped>
.title-group-name {
  font-weight: bold;
  margin-right: 5px;
}
.prefix {
  font-weight: bold;
}
</style>
