<template>
  <div class="name">{{ series.name }}</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: { series: {} },
})
</script>

<style scoped>
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
