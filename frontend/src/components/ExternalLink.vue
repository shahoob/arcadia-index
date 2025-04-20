<template>
  <a target="_blank" :href="link"><img :src="'logos/external_links/' + getLinkLogo()" /></a>
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
      if (this.link.includes('musicbrainz.org')) {
        return 'musicbrainz.svg'
      }
      if (this.link.includes('discogs.com')) {
        return 'discogs.svg'
      }
      if (this.link.includes('imdb.com')) {
        return 'imdb.svg'
      }
      if (this.link.includes('themoviedb.org')) {
        return 'tmdb.svg'
      }
      if (this.link.includes('thetvdb.com')) {
        return 'tvdb.svg'
      }
      if (this.link.includes('store.steampowered.com')) {
        return 'steam.svg'
      }
    },
  },
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
