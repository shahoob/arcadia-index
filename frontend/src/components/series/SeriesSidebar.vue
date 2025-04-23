<template>
  <div id="series-sidebar">
    <Image class="series-covers" :src="series.covers[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <ContentContainer class="header-wrapper" container-title="Description">
      <div class="description">{{ series.description }}</div>
    </ContentContainer>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'

export default defineComponent({
  components: { ContentContainer, Image },
  props: { series: {} },
})
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
