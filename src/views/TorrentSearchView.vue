<template>
  <div v-if="search_results">
    <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
      <div class="title-groups">
        <TitleGroupPreviewCoverOnly
          v-for="title_group in title_groups"
          :key="title_group.id"
          :title_group="title_group"
        />
      </div>
    </ContentContainer>
    <TitleGroupPreviewTable
      v-for="title_group in search_results.title_groups"
      :key="title_group.id"
      :title_group="title_group"
      v-if="title_group_preview_mode == 'table'"
      class="preview-table"
    />
  </div>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import { searchTorrents } from '@/services/api/torrentService'
export default {
  components: {
    ContentContainer,
    TitleGroupPreviewCoverOnly,
    TitleGroupPreviewTable,
  },
  data() {
    return {
      search_results: null,
      searchOptions: { title_group_name: '' },
      title_group_preview_mode: 'table', // TODO: make a select button to switch from cover-only to table
    }
  },
  created() {
    searchTorrents(this.searchOptions).then((data) => {
      this.search_results = data
    })
  },
}
</script>

<style scoped>
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
