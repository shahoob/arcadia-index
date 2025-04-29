<template>
  <div class="title-group-preview-cover-only">
    <a :href="'/title-group/' + id">
      <img class="title-group-cover" :src="cover" v-tooltip.top="name" alt="Title Group Cover" />
    </a>
  </div>
</template>
<script setup lang="ts">
defineProps<{
  id: number
  name: string
  cover: string
}>()
</script>

<style>
.title-group-preview-cover-only .title-group-cover {
  height: 20em;
  border-radius: 7px;
}
.title-group-preview-cover-only a:hover {
  background-color: transparent;
}
</style>
