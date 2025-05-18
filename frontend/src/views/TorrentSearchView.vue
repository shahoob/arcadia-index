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
          v-for="title_group in search_results.title_groups"
          :key="title_group.id"
          :id="title_group.id"
          :name="title_group.name"
          :cover="title_group.covers[0]"
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

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import { searchTorrents, type TorrentSearchResults } from '@/services/api/torrentService'
import TorrentSearchInputs, { type SearchForm } from '@/components/torrent/TorrentSearchInputs.vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const search_results = ref<TorrentSearchResults>()
const title_group_preview_mode = ref<'table' | 'cover-only'>('table') // TODO: make a select button to switch from cover-only to table
const loading = ref(false)
const initialTitleGroupName = ref('')

const search = async (searchForm: SearchForm) => {
  loading.value = true
  search_results.value = await searchTorrents(searchForm)
  loading.value = false
}

onMounted(async () => {
  // TODO: fix the search input not being populated
  initialTitleGroupName.value = route.query.title_group_name?.toString() ?? ''
  search({ title_group_name: initialTitleGroupName.value, tags: '' })
})
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
