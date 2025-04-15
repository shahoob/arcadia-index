<template>
  <div class="title">
    {{ title_group.name }} <span>({{ title_group.original_release_date.substring(0, 4) }})</span>
  </div>
</template>
<script lang="ts">
export default {
  components: {},
  props: {
    title_group: {},
  },
}
</script>
<style scoped>
.title {
  font-weight: bold;
  font-size: 2em;
}
</style>
