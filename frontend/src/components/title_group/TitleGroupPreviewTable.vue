<template>
  <ContentContainer>
    <div class="title-group-preview-table">
      <!-- TODO : add tags and other potentially useful information -->
      <!-- TODO : clicking on a torrent should redirect to the title_group page
      edit the titlegrouptable component to have a prop that allows this -->
      <Image :src="title_group.covers[0]" preview class="cover">
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="right">
        <div class="title">
          <RouterLink :to="`/title-group/${title_group.id}`">{{ title_group.name }}</RouterLink>
          <span class="year">({{ title_group.original_release_date.substring(0, 4) }})</span>
        </div>
        <TitleGroupTable :title_group="title_group" :preview="true" />
      </div>
    </div>
  </ContentContainer>
</template>
<script setup lang="ts">
import { Image } from 'primevue'
import TitleGroupTable from './TitleGroupTable.vue'
import ContentContainer from '../ContentContainer.vue'
import type { TitleGroupHierarchyLite } from '@/services/api/artistService'

defineProps<{
  title_group: TitleGroupHierarchyLite
}>()
</script>
<style scoped>
.title-group-preview-table {
  display: flex;
  justify-content: center;
}
.right {
  width: 100%;
}
.title {
  font-weight: bold;
  font-size: 1.5em;
  margin-bottom: 5px;
}
.year {
  font-size: 0.8em;
  margin-left: 5px;
}
</style>
<style>
.title-group-preview-table .cover {
  height: 20em;
  margin-right: 20px;
}
.title-group-preview-table .cover img {
  border-radius: 7px;
}
</style>
