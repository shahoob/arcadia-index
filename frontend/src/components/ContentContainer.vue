<template>
  <div class="content-container">
    <div class="title" v-if="containerTitle">{{ containerTitle }}</div>
    <slot></slot>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ContentContainer',
  props: { containerTitle: {}, slotClass: {} },
})
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
