<template>
  <div class="content-container">
    <div class="title" v-if="containerTitle">{{ containerTitle }}</div>
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  containerTitle: string
}>()
</script>

<style scoped>
.content-container {
  padding: 12px;
  border-radius: 10px;
  background-color: var(--color-background-secondary);
  display: block !important;
}
.title {
  color: var(--color-primary);
  margin-bottom: 10px;
}
</style>
