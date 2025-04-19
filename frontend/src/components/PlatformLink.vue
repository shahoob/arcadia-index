<template>
  <a v-tooltip.top="title_group.platform" :href="`/title-group?id=${title_group.id}`">
    <img :src="`logos/platforms/${title_group.platform}.svg`" />
  </a>
</template>

<script lang="ts">
export default {
  props: {
    title_group: {},
  },
  methods: {},
}
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
