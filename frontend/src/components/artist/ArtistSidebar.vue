<template>
  <div class="artist-sidebar">
    <Image class="artist-pictures" :src="artist.pictures[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <ContentContainer v-if="artist.description">
      <div class="description">{{ artist.description }}</div>
    </ContentContainer>
  </div>
</template>
<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'
export default {
  components: { ContentContainer, Image },
  props: { artist: {} },
  data() {
    return {}
  },
}
</script>
<style scoped>
.artist-sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.content-container {
  margin-top: 10px;
  width: 100%;
}
.description {
  max-height: 50vh;
  overflow-y: scroll;
}
</style>
<style>
.artist-sidebar .artist-pictures {
  img {
    width: 100%;
    border-radius: 7px;
  }
}
</style>
