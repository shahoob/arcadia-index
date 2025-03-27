<template>
  <!-- TODO: use skeletons while the data is loading -->
  <div id="title-group-view" v-if="title_group">
    <ContentContainer class="header">
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
            <span class="title-metadata"
              >({{ title_group.original_release_date.substring(0, 4) }})</span
            >
          </div>
          <div class="information-line">
            <span class="item-title">Tags:</span>
            <div class="item" v-for="(tag, index) in title_group.tags" :key="tag">
              {{ tag }}<span v-if="index !== title_group.tags.length - 1">,</span>
            </div>
          </div>
          <div class="information-line" v-if="title_group.name_aliases">
            <span class="item-title">Aliases:</span>
            <div v-for="(alias, index) in title_group.name_aliases" :key="alias">
              {{ alias }}<span v-if="index !== title_group.name_aliases.length - 1">,</span>
            </div>
          </div>
          <div class="information-line">
            <span class="item-title">Original language:</span>
            {{ title_group.original_language }}
          </div>
          <div v-if="title_group.series" class="information-line series">
            <span class="item-title">Series:</span>
            <a :href="'/series?id=' + title_group.series.id">{{ title_group.series.name }}</a>
          </div>
          <div class="information-line">
            <div v-for="link in title_group.external_links" :key="link">
              <ExternalLink :link="link" />
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
    <div class="actions">
      <i v-tooltip.top="'Subscribe'" class="pi pi-bell" />
      <i v-tooltip.top="'Bookmark'" class="pi pi-bookmark" />
      <i v-tooltip.top="'Edit'" class="pi pi-pen-to-square" />
      <i v-tooltip.top="'Upload Torrent'" class="pi pi-upload" />
      <i v-tooltip.top="'Request format'" class="pi pi-shopping-cart" />
    </div>
    <ContentContainer>
      <TitleGroupTable :title_group="title_group" />
    </ContentContainer>
    <ContentContainer class="torrent-requests">
      <!-- TODO: Make it foldable and add title to the table (folded by default, hidden if no request) -->
      <TorrentRequestsTable
        :torrent_requests="title_group.torrent_requests"
        :title_group="title_group"
      />
    </ContentContainer>
    <ContentContainer class="description" v-if="title_group">
      <!-- TODO: add bbcode interpreter : https://github.com/JiLiZART/bbob -->
      {{ title_group.description }}
    </ContentContainer>
    <GeneralComments :comments="title_group.comments" />
  </div>
</template>

<script lang="ts">
import GeneralComments from '@/components/community/GeneralComments.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { getTitleGroup } from '@/services/api/torrentService'
import TitleGroupTable from '@/components/torrents/TitleGroupTable.vue'
import { Galleria } from 'primevue'
import Image from 'primevue/image'
import AffiliatedArtist from '@/components/torrents/AffiliatedArtist.vue'
import ExternalLink from '@/components/torrents/ExternalLink.vue'
import TorrentRequestsTable from '@/components/torrents/TorrentRequestsTable.vue'

export default {
  components: {
    ContentContainer,
    TitleGroupTable,
    Galleria,
    Image,
    AffiliatedArtist,
    ExternalLink,
    GeneralComments,
    TorrentRequestsTable,
  },
  setup() {},
  data() {
    return {
      title_group: null,
    }
  },
  created() {
    getTitleGroup(this.$route.query.id?.toString()).then((data) => {
      this.title_group = data
    })
  },
}
</script>

<style scoped>
.header {
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
.affiliated-artists {
  margin-right: 0px;
  margin-left: auto;
}
.actions {
  display: flex;
  justify-content: center;
  margin-bottom: 20px;
}
.actions i {
  margin: 0px 0.5em;
  cursor: pointer;
}
.torrent-requests {
  margin-top: 20px;
}
.description {
  margin-top: 20px;
}
.comments {
  margin-top: 20px;
}
</style>
<style>
#title-group-view .left .p-galleria-content img {
  height: 20em !important;
  border-radius: 7px;
}
#title-group-view .p-galleria-indicator-list {
  padding: 7px !important;
}
</style>
