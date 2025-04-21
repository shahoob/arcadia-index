<template>
  <div id="search-bars">
    <InputText
      type="text"
      :placeholder="$t('torrent.torrent', 2)"
      v-model="searchForm.torrents"
      size="small"
      @keyup.enter="$router.push(`/torrents?title_group_name=${searchForm.torrents}`)"
    />
    <InputText
      type="text"
      :placeholder="$t('artist.artist', 2)"
      v-model="searchForm.artists"
      size="small"
    />
    <InputText
      type="text"
      :placeholder="$t('series.series')"
      v-model="searchForm.series"
      size="small"
    />
    <InputText
      type="text"
      :placeholder="$t('forum.forum', 2)"
      v-model="searchForm.forums"
      size="small"
    />
    <InputText
      type="text"
      :placeholder="$t('user.user', 2)"
      v-model="searchForm.users"
      size="small"
    />
  </div>
</template>

<script lang="ts">
import InputText from 'primevue/inputtext'

export default {
  components: { InputText },
  data() {
    return {
      searchForm: {
        torrents: '',
        artists: '',
        series: '',
        requests: '',
        forums: '',
        users: '',
      },
    }
  },
  methods: {},
}
</script>

<style scoped>
#search-bars {
  display: flex;
  justify-content: center;
  width: 100%;
}
.p-inputtext {
  margin: 0 5px;
  width: 10%;
}
</style>
