<template>
  <Card class="p-card">
    <template #header>
      <h3>Album of the Month</h3>
    </template>
    <template #content>
      <p class="mt-2">This month's featured album!</p>
    </template>
  </Card>
</template>

<script lang="ts">
import Card from 'primevue/card'

export default {
  components: { Card },
}
</script>

<style scoped>
.p-card {
  background-color: #2c2c2c;
  color: white;
}
.album-cover {
  width: 100%;
}
</style>
