<template>
  <div class="name">{{ artist.name }}</div>
</template>
<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  components: {},
  props: { artist: {} },
  data() {
    return {}
  },
})
</script>
<style scoped>
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
