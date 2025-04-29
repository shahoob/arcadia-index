<template>
  <Galleria
    v-model:visible="modalVisible"
    v-model:activeIndex="activeIndex"
    :value="images"
    circular
    fullScreen
    :showItemNavigators="true"
    :showThumbnails="false"
  >
    <template #item="slotProps">
      <img :src="slotProps.item" class="fullscreen-image" />
    </template>
  </Galleria>

  <div class="thumbnails">
    <img
      v-for="(image, index) of images"
      :key="index"
      class="thumbnail"
      :src="image"
      style="cursor: pointer"
      @click="imageClicked(index)"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Galleria } from 'primevue'

defineProps<{
  images: string[]
}>()

const modalVisible = ref(false)
const activeIndex = ref(0)

const imageClicked = (index: number) => {
  activeIndex.value = index
  modalVisible.value = true
}
</script>

<style scoped>
.thumbnails {
  display: flex;
  justify-content: center;
}
.thumbnail {
  width: 25%;
  margin: 0.3%;
  border-radius: 7px;
}
.fullscreen-image {
  max-width: 95vw;
  max-height: 95vh;
}
</style>
