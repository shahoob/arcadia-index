<template>
  <a target="_blank" :href="link"><img :src="'logos/' + getLinkLogo()" /></a>
</template>

<script lang="ts">
export default {
  props: {
    link: {},
  },
  methods: {
    getLinkLogo() {
      if (this.link.includes('wikipedia.org')) {
        return 'wikipedia.png'
      }
      if (this.link.includes('openlibrary.org')) {
        return 'open-library.svg'
      }
    },
  },
}
</script>
<style scoped>
img {
  height: 30px;
}
a {
  background-color: transparent;
}
</style>
