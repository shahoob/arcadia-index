<template>
  <a
    v-tooltip.top="title_group.platform ?? title_group.name"
    :href="`/title-group?id=${title_group.id}`"
  >
    <img :src="`logos/platforms/${title_group.platform ?? title_group.content_type}.svg`" />
  </a>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    title_group: {},
  },
  methods: {},
})
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
