<template>
  <div id="series-view" v-if="series">
    <ContentContainer class="header">
      <!-- TODO: implement carousel for multiple covers -->
      <Image class="series-covers" :src="series.covers[0]" preview>
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="textual-information">
        <div class="name">{{ series.name }}</div>
        <div class="description">{{ series.description }}</div>
      </div>
    </ContentContainer>
    <ContentContainer class="title-groups">
      <TitleGroupPreviewCoverOnly
        v-for="title_group in title_groups"
        :key="title_group.id"
        :title_group="title_group"
        v-if="title_group_preview_mode == 'cover-only'"
      />
      <TitleGroupPreviewTable
        v-for="title_group in title_groups"
        :key="title_group.id"
        :title_group="title_group"
        v-if="title_group_preview_mode == 'table'"
      />
    </ContentContainer>
  </div>
</template>
<script lang="ts">
import { getSeries } from '@/services/api/seriesService'
import ContentContainer from '@/components/ContentContainer.vue'
import { Image } from 'primevue'
import TitleGroupPreviewCoverOnly from '@/components/torrents/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/torrents/TitleGroupPreviewTable.vue'
export default {
  components: { ContentContainer, Image, TitleGroupPreviewCoverOnly, TitleGroupPreviewTable },
  data() {
    return {
      series: null,
      title_groups: [],
      title_group_preview_mode: 'table', // TODO: make a select button to switch from cover-only to table
    }
  },
  created() {
    getSeries(this.$route.query.id).then((data) => {
      this.series = data.series
      this.title_groups = data.title_groups
    })
  },
}
</script>

<style scoped>
.header {
  display: flex;
  margin-bottom: 15px;
}
.series-covers {
  margin-right: 10px;
}
.name {
  font-weight: bold;
  font-size: 2em;
}
.title-groups {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}
</style>
<style>
#series-view .series-covers img {
  width: 30em;
  border-radius: 7px;
}
</style>
