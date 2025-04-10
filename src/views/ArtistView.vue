<template>
  <div id="artist-view" v-if="artist">
    <ContentContainer class="header">
      <Image class="artist-pictures" :src="artist.pictures[0]" preview>
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="textual-information">
        <div class="name">{{ artist.name }}</div>
        <div class="description">{{ artist.description }}</div>
      </div>
    </ContentContainer>
    <ContentContainer v-if="title_group_preview_mode == 'cover-only'" class="title-groups">
      <TitleGroupPreviewCoverOnly
        v-for="title_group in title_groups"
        :key="title_group.id"
        :title_group="title_group"
      />
    </ContentContainer>
    <TitleGroupPreviewTable
      v-for="title_group in title_groups"
      :key="title_group.id"
      :title_group="title_group"
      v-if="title_group_preview_mode == 'table'"
      class="preview-table"
    />
  </div>
</template>
<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'
import TitleGroupPreviewCoverOnly from '@/components/torrents/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/torrents/TitleGroupPreviewTable.vue'
import { getArtist } from '@/services/api/artistService'
export default {
  components: { ContentContainer, Image, TitleGroupPreviewCoverOnly, TitleGroupPreviewTable },
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
.header {
  display: flex;
  margin-bottom: 15px;
  height: 35vh;
}
.artist-pictures {
  margin-right: 10px;
}
.name {
  font-weight: bold;
  font-size: 2em;
}
.description {
  margin-top: 10px;
  max-height: 80%;
  overflow-y: scroll;
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
<style>
#artist-view .artist-pictures img {
  max-width: 20em;
  max-height: 20em;
  border-radius: 7px;
}
</style>
