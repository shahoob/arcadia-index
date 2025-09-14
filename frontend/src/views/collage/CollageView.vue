<template>
  <div v-if="collageAndAssociatedData" id="collage-view">
    <div class="main-content">
      <div class="title">{{ collageAndAssociatedData.collage.name }}</div>
      <TitleGroupList
        v-if="collageAndAssociatedData.collage.collage_type === 'TitleGroup'"
        :titleGroups="collageAndAssociatedData.entries.map((entry) => entry.title_group as TitleGroupHierarchyLite)"
        :titleGroupPreview
      />
      <!-- TODO: display Artists, Entities and Master Groups -->
    </div>
    <CollageSidebar :collage="collageAndAssociatedData.collage" />
  </div>
</template>
<script setup lang="ts">
import { getCollage, type CollageAndAssociatedData } from '@/services/api/collageService'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import CollageSidebar from '@/components/collage/CollageSidebar.vue'
import TitleGroupList, { type titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import type { TitleGroupHierarchyLite } from '@/services/api/artistService'

const route = useRoute()
const siteName = import.meta.env.VITE_SITE_NAME
const collageAndAssociatedData = ref<CollageAndAssociatedData>()
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table

onMounted(async () => {
  await fetchCollage()
})

const fetchCollage = async () => {
  collageAndAssociatedData.value = await getCollage(parseInt(route.params.id.toString()))
  document.title = `${collageAndAssociatedData.value.collage.name} - ${siteName}`
}
</script>
<style scoped>
#collage-view {
  display: flex;
}
.main-content {
  width: 80%;
  margin-right: 10px;
}
</style>
