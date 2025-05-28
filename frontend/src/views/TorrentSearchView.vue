<template>
  <div v-if="search_results">
    <TorrentSearchInputs
      class="torrent-search-inputs"
      @search="search"
      :loading
      :initialForm
      :showStaffOptions="userStore.class === 'staff'"
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
import {
  searchTorrentsLite,
  type TorrentSearch,
  type TorrentSearchResults,
} from '@/services/api/torrentService'
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
  title_group: { name: '', include_empty_groups: true },
  torrent: {},
  page: 1,
  page_size: 20,
  sort_by: 'torrent_created_at',
  order: 'desc',
})

const search = async (torrentSearch: TorrentSearch) => {
  loading.value = true
  search_results.value = await searchTorrentsLite(torrentSearch)
  loading.value = false
}

const loadSearchForm = async () => {
  initialForm.value.title_group.name = route.query.title_group_name?.toString() ?? ''
  if (userStore.class === 'staff') {
    initialForm.value.torrent.staff_checked = false
    initialForm.value.torrent.reported = null
  }
  search(initialForm.value)
}

onMounted(async () => {
  loadSearchForm()
})

watch(() => route.query, loadSearchForm, { immediate: true })
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
