<template>
  <div v-if="search_results">
    <TorrentSearchInputs
      class="torrent-search-inputs"
      @search="search"
      :loading
      :initialTitleGroupName
    />
    <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
      <div class="title-groups">
        <TitleGroupPreviewCoverOnly
          v-for="title_group in title_groups"
          :key="title_group.id"
          :title_group="title_group"
        />
      </div>
    </ContentContainer>
    <div v-if="title_group_preview_mode == 'table'">
      <TitleGroupPreviewTable
        v-for="title_group in search_results.title_groups"
        :key="title_group.id"
        :title_group="title_group"
        class="preview-table"
      />
    </div>
  </div>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import { searchTorrents } from '@/services/api/torrentService'
import TorrentSearchInputs from '@/components/torrent/TorrentSearchInputs.vue'
export default {
  components: {
    ContentContainer,
    TitleGroupPreviewCoverOnly,
    TitleGroupPreviewTable,
    TorrentSearchInputs,
  },
  data() {
    return {
      search_results: null,
      title_group_preview_mode: 'table', // TODO: make a select button to switch from cover-only to table
      loading: false,
      initialTitleGroupName: '',
    }
  },
  methods: {
    search(searchForm) {
      this.loading = true
      searchTorrents(searchForm).then((data) => {
        this.search_results = data
        this.loading = false
      })
    },
  },
  created() {
    this.initialTitleGroupName = this.$route.query.title_group_name ?? ''
    this.search({ title_group_name: this.initialTitleGroupName })
  },
}
</script>

<style scoped>
.torrent-search-inputs {
  margin-bottom: 25px;
}
.title-groups {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}
.preview-table {
  margin-bottom: 15px;
}
</style>
