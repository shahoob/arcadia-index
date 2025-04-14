<template>
  <div class="content-container">
    <div class="title" v-if="containerTitle">{{ containerTitle }}</div>
    <div>
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'ContentContainer',
  props: { containerTitle: {}, slotClass: {} },
}
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
