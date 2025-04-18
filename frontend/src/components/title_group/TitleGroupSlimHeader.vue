<template>
  <div class="title">
    <span v-if="title_group.series?.id">
      <a class="series-name" :href="`/series?id=${title_group.series.id}`"
        >{{ title_group.series.name }}
      </a>
      -
    </span>
    <span class="title-group-name">{{ title_group.name }}</span>
    <span class="year">({{ title_group.original_release_date.substring(0, 4) }})</span>
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
  font-size: 2em;
}
.title-group-name {
  font-weight: bold;
  margin-right: 5px;
}
.series-name {
  font-weight: bold;
}
</style>
