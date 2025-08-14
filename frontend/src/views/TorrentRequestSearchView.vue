<template>
  <div v-if="searchResults">
    <TorrentRequestSearchInputs
      ref="searchInputsRef"
      class="torrent-request-search-inputs"
      @search="search"
      :loading
      :initialForm
    />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
    <TorrentRequestsTable :torrentRequests="searchResults" displayTitleGroup />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ResultsPagination from '@/components/ResultsPagination.vue'
import TorrentRequestsTable from '@/components/torrent_request/TorrentRequestsTable.vue'
import TorrentRequestSearchInputs from '@/components/torrent_request/TorrentRequestSearchInputs.vue'
import { useRoute } from 'vue-router'
import { watch } from 'vue'
import type { VNodeRef } from 'vue'
import { searchTorrentRequests, type SearchTorrentRequestsQuery, type TorrentRequestWithTitleGroupLite } from '@/services/api/torrentRequestService'

const route = useRoute()

const searchInputsRef = ref<VNodeRef | null>(null)

const searchResults = ref<TorrentRequestWithTitleGroupLite[]>()
const loading = ref(false)
const initialForm = ref<SearchTorrentRequestsQuery>({
  title_group_name: null,
  tags: null,
  page: 1,
  page_size: 20,
})

const search = async (form: SearchTorrentRequestsQuery) => {
  loading.value = true
  searchResults.value = await searchTorrentRequests(form).finally(() => {
    loading.value = false
  })
}

const loadSearchForm = async () => {
  initialForm.value.title_group_name = route.query.title_group_name?.toString() ?? null
  initialForm.value.tags = route.query.tags ? (Array.isArray(route.query.tags) ? route.query.tags : [route.query.tags.toString()]) : null

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
.torrent-request-search-inputs {
  margin-bottom: 25px;
}
</style>
