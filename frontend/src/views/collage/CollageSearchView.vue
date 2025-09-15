<template>
  <CollageSearchForm @gotResults="gotResults" style="margin-bottom: 15px" />
  <CollagesTable :collages />
</template>
<script setup lang="ts">
import type { CollageSearchResponse, CollageSearchResult } from '@/services/api/collageService'
import { ref } from 'vue'
import CollageSearchForm from '@/components/collage/CollageSearchForm.vue'
import CollagesTable from '@/components/collage/CollagesTable.vue'

const collages = ref<CollageSearchResult[]>([])

const gotResults = (collageSearchResponse: CollageSearchResponse) => {
  collages.value = collageSearchResponse.results
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
