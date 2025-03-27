<template>
  <div class="title-group-preview-table">
    <TitleGroupTable :title_group="title_group" :edition_groups="title_group.edition_groups" />
  </div>
</template>
<script lang="ts">
import { Image } from 'primevue'
import TitleGroupTable from './TitleGroupTable.vue'
export default {
  components: { Image, TitleGroupTable },
  props: {
    title_group: {},
  },
  methods: {
    titleGroupClicked() {},
  },
}
</script>

<style>
.title-group-preview-cover-only .title-group-cover {
  height: 20em;
  border-radius: 7px;
}
.title-group-preview-cover-only a:hover {
  background-color: transparent;
}
</style>
