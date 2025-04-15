<template>
  <ContentContainer id="series-full-header" class="header-wrapper">
    <div class="header">
      <Image class="series-covers" :src="series.covers[0]" preview>
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="textual-information">
        <div class="name">{{ series.name }}</div>
        <div class="description">{{ series.description }}</div>
      </div>
    </div>
  </ContentContainer>
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
.header-wrapper {
  margin-bottom: 15px;
}
.header {
  display: flex;
}
.series-covers {
  margin-right: 10px;
}
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
<style>
#series-full-header .series-covers img {
  max-width: 30em;
  max-height: 30em;
  border-radius: 7px;
}
</style>
