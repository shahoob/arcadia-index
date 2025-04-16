<template>
  <div
    id="series-view"
    v-if="series"
    :class="{
      'sidebar-right': userStore.settings.site_appearance.item_detail_layout == 'sidebar_right',
      'sidebar-left': userStore.settings.site_appearance.item_detail_layout == 'sidebar_left',
    }"
  >
    <div class="main">
      <SeriesFullHeader
        :series
        v-if="userStore.settings.site_appearance.item_detail_layout == 'header'"
      />
      <SeriesSlimHeader v-else class="slim-header" :series />
      <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
        <div class="title-groups">
          <TitleGroupPreviewCoverOnly
            v-for="title_group in title_groups"
            :key="title_group.id"
            :title_group="title_group"
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
    <SeriesSidebar :series />
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
</script>
<script lang="ts">
import { getSeries } from '@/services/api/seriesService'
import SeriesSlimHeader from '@/components/series/SeriesSlimHeader.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import SeriesFullHeader from '@/components/series/SeriesFullHeader.vue'
import SeriesSidebar from '@/components/series/SeriesSidebar.vue'
export default {
  components: {
    SeriesSidebar,
    SeriesFullHeader,
    ContentContainer,
    TitleGroupPreviewCoverOnly,
    TitleGroupPreviewTable,
    SeriesSlimHeader,
  },
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
#series-view .series-covers img {
  border-radius: 7px;
}
</style>
