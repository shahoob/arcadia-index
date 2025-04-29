<template>
  <a target="_blank" :href="link"><img :src="'/logos/external_links/' + getLinkLogo()" /></a>
</template>

<script setup lang="ts">
const props = defineProps<{
  link: string
}>()
const getLinkLogo = () => {
  if (props.link.includes('wikipedia.org')) {
    return 'wikipedia.png'
  }
  if (props.link.includes('openlibrary.org')) {
    return 'open-library.svg'
  }
  if (props.link.includes('musicbrainz.org')) {
    return 'musicbrainz.svg'
  }
  if (props.link.includes('discogs.com')) {
    return 'discogs.svg'
  }
  if (props.link.includes('imdb.com')) {
    return 'imdb.svg'
  }
  if (props.link.includes('themoviedb.org')) {
    return 'tmdb.svg'
  }
  if (props.link.includes('thetvdb.com')) {
    return 'tvdb.svg'
  }
  if (props.link.includes('store.steampowered.com')) {
    return 'steam.svg'
  }
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
