<template>
  <div class="title-group-preview">
    <a :href="'/title-group?id=' + title_group.id">
      <img
        class="title-group-cover"
        :src="title_group.covers[0]"
        v-tooltip.top="title_group.name"
        alt="Title Group Cover"
      />
    </a>
  </div>
</template>
<script lang="ts">
import { Image } from 'primevue'
export default {
  components: { Image },
  props: {
    title_group: {},
  },
  methods: {
    titleGroupClicked() {},
  },
}
</script>

<style>
.title-group-preview .title-group-cover {
  height: 20em;
  border-radius: 7px;
}
.title-group-preview a:hover {
  background-color: transparent;
}
</style>
