<template>
  <div v-if="search_results">
    <TorrentSearchInputs class="torrent-search-inputs" @search="search" :loading :initialForm :showStaffOptions="userStore.class === 'staff'" />
    <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
      <div class="title-groups">
        <TitleGroupPreviewCoverOnly v-for="title_group in search_results.title_groups" :key="title_group.id" :titleGroup="title_group" />
      </div>
    </ContentContainer>
    <div v-if="title_group_preview_mode == 'table'">
      <TitleGroupPreviewTable v-for="title_group in search_results.title_groups" :key="title_group.id" :title_group="title_group" class="preview-table" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import { searchTorrentsLite, type TorrentSearch, type TorrentSearchResults } from '@/services/api/torrentService'
import TorrentSearchInputs from '@/components/torrent/TorrentSearchInputs.vue'
import { useRoute } from 'vue-router'
import { watch } from 'vue'
import { useUserStore } from '@/stores/user'

const route = useRoute()
const userStore = useUserStore()

const search_results = ref<TorrentSearchResults>()
const title_group_preview_mode = ref<'table' | 'cover-only'>('table') // TODO: make a select button to switch from cover-only to table
const loading = ref(false)
const initialForm = ref<TorrentSearch>({
  title_group: { name: '', include_empty_groups: false },
  torrent: {},
  page: 1,
  page_size: 5,
  sort_by: 'torrent_created_at',
  order: 'desc',
})
const siteName = import.meta.env.VITE_SITE_NAME

const search = async (torrentSearch: TorrentSearch) => {
  loading.value = true
  search_results.value = await searchTorrentsLite(torrentSearch).finally(() => {
    loading.value = false
  })
}

const loadSearchForm = async () => {
  initialForm.value.title_group.name = route.query.title_group_name?.toString() ?? ''
  initialForm.value.torrent.created_by_id = parseInt(route.query.created_by_id as string) ?? null
  initialForm.value.torrent.snatched_by_id = parseInt(route.query.snatched_by_id as string) ?? null
  if (userStore.class === 'staff') {
    initialForm.value.torrent.staff_checked = false
    initialForm.value.torrent.reported = null
  }
  search(initialForm.value)
}

onMounted(async () => {
  loadSearchForm()
  document.title = `Torrent Search - ${siteName}`
})

watch(
  () => route.query,
  (newQuery, oldQuery) => {
    if (oldQuery !== undefined) {
      loadSearchForm()
    }
  },
  { immediate: true },
)
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
