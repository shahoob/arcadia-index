<template>
  <div
    v-if="artist"
    id="artist-view"
    :class="{
      'sidebar-right': userStore.settings.site_appearance.item_detail_layout == 'sidebar_right',
      'sidebar-left': userStore.settings.site_appearance.item_detail_layout == 'sidebar_left',
    }"
  >
    <div class="main">
      <ArtistFullHeader
        :artist
        v-if="userStore.settings.site_appearance.item_detail_layout == 'header'"
      />
      <ArtistSlimHeader v-else class="slim-header" :artist />
      <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
        <div class="title-groups">
          <TitleGroupPreviewCoverOnly
            v-for="title_group in title_groups"
            :key="title_group.id"
            :id="title_group.id"
            :name="title_group.name"
            :cover="title_group.covers[0]"
          />
        </div>
      </ContentContainer>
      <div v-if="title_group_preview_mode == 'table'">
        <TitleGroupPreviewTable
          v-for="title_group in title_groups"
          :key="title_group.id"
          :title_group="title_group"
          class="preview-table"
        />
      </div>
    </div>
    <ArtistSidebar :artist />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
import ContentContainer from '@/components/ContentContainer.vue'
import ArtistSidebar from '@/components/artist/ArtistSidebar.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import ArtistFullHeader from '@/components/artist/ArtistFullHeader.vue'
import ArtistSlimHeader from '@/components/artist/ArtistSlimHeader.vue'
import { getArtist, type Artist, type TitleGroupHierarchyLite } from '@/services/api/artistService'

const route = useRoute()
const userStore = useUserStore()

const artist = ref<Artist>()
const title_groups = ref<TitleGroupHierarchyLite[]>([])
const title_group_preview_mode = ref<'table' | 'cover-only'>('table')

onMounted(async () => {
  const artistData = await getArtist(parseInt(route.params.id.toString()))
  artist.value = artistData.artist
  title_groups.value = artistData.title_groups
})
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
