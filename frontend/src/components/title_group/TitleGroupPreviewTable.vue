<template>
  <ContentContainer>
    <div class="title-group-preview-table">
      <!-- TODO : add tags and other potentially useful information -->
      <!-- TODO : clicking on a torrent should redirect to the title_group page
      edit the titlegrouptable component to have a prop that allows this -->
      <Image :src="title_group.covers[0]" preview class="cover">
        <template #previewicon>
          <i class="pi pi-search"></i>
        </template>
      </Image>
      <div class="right">
        <div class="title">
          <a :href="'/title-group?id=' + title_group.id">{{ title_group.name }}</a>
          <span class="year">({{ title_group.original_release_date.substring(0, 4) }})</span>
        </div>
        <TitleGroupTable
          class="table"
          :title_group="title_group"
          :edition_groups="title_group.edition_groups"
          :preview="true"
        />
      </div>
    </div>
  </ContentContainer>
</template>
<script lang="ts">
import { defineComponent } from 'vue'
import { Image } from 'primevue'
import TitleGroupTable from './TitleGroupTable.vue'
import ContentContainer from '../ContentContainer.vue'

export default defineComponent({
  components: { Image, TitleGroupTable, ContentContainer },
  props: {
    title_group: {},
  },
  methods: {
    titleGroupClicked() {},
  },
})
</script>
<style scoped>
.title-group-preview-table {
  display: flex;
  justify-content: center;
}
.right {
  width: 100%;
}
.title {
  font-weight: bold;
  font-size: 1.5em;
  margin-bottom: 5px;
}
.year {
  font-size: 0.8em;
  margin-left: 5px;
}
.table {
  width: 100%;
}
</style>
<style>
.title-group-preview-table .cover {
  height: 20em;
  margin-right: 20px;
}
.title-group-preview-table .cover img {
  border-radius: 7px;
}
</style>
