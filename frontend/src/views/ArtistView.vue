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
            :title_group="title_group"
          />
        </div>
      </ContentContainer>
      <TitleGroupPreviewTable
        v-for="title_group in title_groups"
        :key="title_group.id"
        :title_group="title_group"
        v-if="title_group_preview_mode == 'table'"
        class="preview-table"
      />
    </div>
    <ArtistSidebar :artist />
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
</script>
<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import ArtistSidebar from '@/components/artist/ArtistSidebar.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import ArtistFullHeader from '@/components/artist/ArtistFullHeader.vue'
import ArtistSlimHeader from '@/components/artist/ArtistSlimHeader.vue'
import { getArtist } from '@/services/api/artistService'
export default {
  components: {
    ArtistFullHeader,
    ContentContainer,
    TitleGroupPreviewCoverOnly,
    TitleGroupPreviewTable,
    ArtistSidebar,
    ArtistSlimHeader,
  },
  data() {
    return {
      artist: null,
      title_groups: [],
      title_group_preview_mode: 'table', // TODO: make a select button to switch from cover-only to table
    }
  },
  created() {
    getArtist(this.$route.query.id).then((data) => {
      this.artist = data.artist
      this.title_groups = data.title_groups
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
