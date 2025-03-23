<template>
  <div>
    <ContentContainer class="header">
      <Galleria
        :value="title_group.covers"
        :numVisible="1"
        :circular="true"
        :showItemNavigators="false"
        :showThumbnails="false"
        :showIndicators="true"
      >
        <template #item="slotProps">
          <Image class="title-group-view-cover" :src="slotProps.item" preview>
            <template #previewicon>
              <i class="pi pi-search"></i>
            </template>
            <!-- <template #image>
              <img :src="slotProps.item" alt="cover preview" />
            </template>
            <template #preview="slotProps">
              {{ slotProps }}
              <img :src="slotProps.item" :alt="slotProps" />
            </template> -->
          </Image>
        </template>
      </Galleria>
      <div class="textual-information">
        <div class="title">{{ title_group.name }}</div>
        <div class="tags">
          <div class="tag" v-for="(tag, index) in title_group.tags" :key="tag">
            {{ tag }}<span v-if="index !== title_group.tags.length - 1">,</span>
          </div>
        </div>
      </div>
    </ContentContainer>
    <ContentContainer>
      <TitleGroupTable :edition_groups="edition_groups" />
    </ContentContainer>
  </div>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { getTitleGroup } from '@/services/api/torrentService'
import TitleGroupTable from '@/components/torrents/TitleGroupTable.vue'
import { Galleria } from 'primevue'
import Image from 'primevue/image'

export default {
  components: { ContentContainer, TitleGroupTable, Galleria, Image },
  setup() {},
  data() {
    return {
      title_group: {},
      edition_groups: [],
    }
  },
  created() {
    getTitleGroup(this.$route.query.id?.toString()).then((data) => {
      this.title_group = data.title_group
      this.edition_groups = data.edition_groups
    })
  },
}
</script>

<style scoped>
.header {
  margin-bottom: 20px;
  display: flex;
}
.p-galleria {
  height: 20em;
  border: none;
  margin-right: 20px;
}
.title {
  font-weight: bold;
  font-size: 2em;
}
.tags {
  display: flex;
}
.tag {
  margin-right: 5px;
}
</style>
<style>
.title-group-view-cover img {
  height: 20em !important;
}
</style>
