<template>
  <div class="container">
    <slot></slot>
  </div>
</template>

<script lang="ts">
export default {
  name: 'ContentContainer',
}
</script>

<style scoped>
.container {
  padding: 7px;
  border-radius: 10px;
  background-color: rgb(29, 29, 29);
}
</style>
