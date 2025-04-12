<template>
  <div class="name">{{ series.name }}</div>
</template>

<script lang="ts">
export default {
  props: { series: {} },
}
</script>

<style scoped>
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
<style>
#series-view .series-covers img {
  max-width: 30em;
  max-height: 30em;
  border-radius: 7px;
}
</style>
