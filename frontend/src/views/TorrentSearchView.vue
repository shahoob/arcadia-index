<template>
  <div v-if="search_results">
    <TorrentSearchInputs
      ref="searchInputsRef"
      class="torrent-search-inputs"
      @search="search"
      :loading
      :initialForm
      :showStaffOptions="userStore.class === 'staff'"
    />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
    <TitleGroupList :titleGroups="search_results.title_groups" :titleGroupPreview />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ResultsPagination from '@/components/ResultsPagination.vue'
import { searchTorrentsLite, type TorrentSearch, type TorrentSearchResults } from '@/services/api/torrentService'
import TorrentSearchInputs from '@/components/torrent/TorrentSearchInputs.vue'
import TitleGroupList from '@/components/title_group/TitleGroupList.vue'
import type { titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import { useRoute } from 'vue-router'
import { watch } from 'vue'
import { useUserStore } from '@/stores/user'
import type { VNodeRef } from 'vue'

const route = useRoute()
const userStore = useUserStore()

const searchInputsRef = ref<VNodeRef | null>(null)

const search_results = ref<TorrentSearchResults>()
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table
const loading = ref(false)
const initialForm = ref<TorrentSearch>({
  title_group: { name: '', include_empty_groups: false },
  torrent: {},
  page: 1,
  page_size: 20,
  sort_by: 'torrent_created_at',
  order: 'desc',
})

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
</style>
