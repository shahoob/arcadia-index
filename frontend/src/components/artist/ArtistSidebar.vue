<template>
  <div class="artist-sidebar">
    <Image class="artist-pictures" :src="artist.pictures[0] ?? '/default_artist_picture.svg'" :preview="artist.pictures.length !== 0">
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <ContentContainer v-if="artist.description" :container-title="t('general.description')">
      <div class="description">
        <BBCodeRenderer :content="artist.description" />
      </div>
    </ContentContainer>
  </div>
</template>
<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import { type Artist } from '@/services/api/artistService'
import { Image } from 'primevue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  artist: Artist
}>()
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
