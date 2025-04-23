<template>
  <ContentContainer class="header-wrapper artist-full-header">
    <div class="header">
      <Image class="artist-pictures" :src="artist.pictures[0]" preview>
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="textual-information">
        <div class="name">{{ artist.name }}</div>
        <div class="description">{{ artist.description }}</div>
      </div>
    </div>
  </ContentContainer>
</template>
<script lang="ts">
import { defineComponent } from 'vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'

export default defineComponent({
  components: { ContentContainer, Image },
  props: { artist: {} },
  data() {
    return {}
  },
})
</script>
<style scoped>
.header-wrapper {
  margin-bottom: 15px;
}
.header {
  display: flex;
  height: 35vh;
}
.artist-pictures {
  margin-right: 10px;
}
.name {
  font-weight: bold;
  font-size: 2em;
}
.description {
  margin-top: 10px;
  max-height: 80%;
  overflow-y: scroll;
}
</style>
<style>
.artist-full-header .artist-pictures img {
  max-width: 20em;
  max-height: 20em;
  border-radius: 7px;
}
</style>
