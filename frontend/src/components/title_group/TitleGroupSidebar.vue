<template>
  <div id="title-group-sidebar">
    <Galleria
      :value="title_group.covers"
      :numVisible="1"
      :circular="true"
      :showItemNavigators="false"
      :showThumbnails="false"
      :showIndicators="true"
      class="carousel"
    >
      <template #item="slotProps">
        <Image :src="slotProps.item" preview>
          <template #previewicon>
            <i class="pi pi-search"></i>
          </template>
        </Image>
      </template>
    </Galleria>
    <ContentContainer container-title="Links">
      <div class="external-links">
        <ExternalLink v-for="link in title_group.external_links" :key="link.url" :link="link" />
      </div>
    </ContentContainer>
    <ContentContainer container-title="Artists">
      <div class="affiliated-artists">
        <AffiliatedArtist
          v-for="artist in title_group.affiliated_artists"
          :key="artist.id"
          :affiliated_artist="artist"
        />
      </div>
    </ContentContainer>
    <ContentContainer container-title="Series" v-if="title_group.series.id">
      <a :href="'/series?id=' + title_group.series.id">{{ title_group.series.name }}</a>
    </ContentContainer>
    <ContentContainer container-title="Tags">
      <div class="tags">
        <div v-for="tag in title_group.tags" :key="tag">{{ tag }}</div>
      </div>
    </ContentContainer>
  </div>
</template>
<script lang="ts">
import { Galleria } from 'primevue'
import Image from 'primevue/image'
import AffiliatedArtist from '@/components/artist/AffiliatedArtist.vue'
import ExternalLink from '@/components/ExternalLink.vue'
import ContentContainer from '../ContentContainer.vue'

export default {
  components: {
    Galleria,
    Image,
    AffiliatedArtist,
    ExternalLink,
    ContentContainer,
  },
  props: {
    title_group: {},
  },
}
</script>
<style scoped>
#title-group-sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  .content-container {
    width: 100%;
    margin-top: 20px;
  }
}
.p-galleria {
  border: none;
}
.external-links {
  display: flex;
  justify-content: center;
  align-items: center;
  a {
    margin: 0px 10px;
  }
}
</style>
<style>
#title-group-sidebar .p-galleria-content img {
  /* height: 20em !important; */
  width: 100% !important;
  border-radius: 7px;
}
</style>
