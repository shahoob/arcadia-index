<template>
  <ContentContainer id="title-group-header">
    <div class="left">
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
      <div class="textual-information">
        <div class="title">
          {{ title_group.name }}
          <span class="title-metadata">
            ({{ title_group.original_release_date.substring(0, 4) }})
          </span>
        </div>
        <div class="information-line">
          <span class="item-title">{{ $t('general.tags') }}:</span>
          <div class="item" v-for="(tag, index) in title_group.tags" :key="tag">
            {{ tag }}<span v-if="index !== title_group.tags.length - 1">,</span>
          </div>
        </div>
        <div
          class="information-line"
          v-if="title_group.name_aliases.length != 0 && title_group.name_aliases[0] != ''"
        >
          <span class="item-title">{{ $t('general.alias', 2) }}:</span>
          <div v-for="(alias, index) in title_group.name_aliases" :key="alias">
            {{ alias }}<span v-if="index !== title_group.name_aliases.length - 1">,</span>
          </div>
        </div>
        <div class="information-line">
          <span class="item-title">{{ $t('general.original_language') }}:</span>
          {{ title_group.original_language }}
        </div>
        <div class="information-line">
          <span class="item-title">{{ $t('general.country') }}:</span>
          {{ title_group.country_from }}
        </div>
        <div v-if="title_group.series.id" class="information-line series">
          <span class="item-title">{{ $t('general.series') }}:</span>
          <a :href="'/series?id=' + title_group.series.id">{{ title_group.series.name }}</a>
        </div>
        <div class="information-line link-logos">
          <div v-for="link in title_group.external_links" :key="link">
            <ExternalLink :link="link" class="link-logo" />
          </div>
        </div>
      </div>
    </div>
    <div class="right">
      <div class="affiliated_artists">
        <AffiliatedArtist
          v-for="affiliated_artist in title_group.affiliated_artists"
          :key="affiliated_artist.artist.id"
          :affiliated_artist="affiliated_artist"
        />
      </div>
    </div>
  </ContentContainer>
</template>
<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'

import { Galleria } from 'primevue'
import Image from 'primevue/image'
import AffiliatedArtist from '@/components/artist/AffiliatedArtist.vue'
import ExternalLink from '@/components/ExternalLink.vue'

export default {
  components: {
    ContentContainer,
    Galleria,
    Image,
    AffiliatedArtist,
    ExternalLink,
  },
  props: {
    title_group: {},
  },
}
</script>
<style scoped>
#title-group-header {
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
}
.left {
  display: flex;
}
.p-galleria {
  height: 22em;
  border: none;
  margin-right: 20px;
}
.title {
  font-weight: bold;
  font-size: 2em;
}
.item-title {
  font-weight: bold;
  margin-right: 7px;
}
.information-line {
  display: flex;
  margin-top: 8px;
}
.information-line .item {
  margin-right: 5px;
}
.series a {
  padding: 0px;
}
.link-logos {
  margin-top: 25px;
  display: flex;
  align-items: center;
}
.link-logo {
  margin-right: 7px;
}
.affiliated-artists {
  margin-right: 0px;
  margin-left: auto;
}
</style>
<style>
#title-group-header .left .p-galleria-content img {
  height: 20em !important;
  border-radius: 7px;
}
#title-group-header .p-galleria-indicator-list {
  padding: 7px !important;
}
</style>
