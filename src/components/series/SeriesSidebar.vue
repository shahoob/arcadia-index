<template>
  <div id="series-sidebar">
    <Image class="series-covers" :src="series.covers[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <ContentContainer class="header-wrapper">
      <div class="description">{{ series.description }}</div>
    </ContentContainer>
  </div>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'
export default {
  components: { ContentContainer, Image },
  props: { series: {} },
}
</script>

<style scoped>
#series-sidebar {
  display: flex;
  flex-direction: column;
}
.content-container {
  margin-top: 10px;
}
</style>
<style>
#series-view .series-covers img {
  width: 100%;
  border-radius: 7px;
}
</style>
