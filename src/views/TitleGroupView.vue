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
          <div class="information-line" v-if="title_group.name_aliases.length != 0">
            <span class="item-title">Aliases:</span>
            <div v-for="(alias, index) in title_group.name_aliases" :key="alias">
              {{ alias }}<span v-if="index !== title_group.name_aliases.length - 1">,</span>
            </div>
          </div>
          <div class="information-line">
            <span class="item-title">Original language:</span>
            {{ title_group.original_language }}
          </div>
          <div class="information-line">
            <span class="item-title">Country from:</span>
            {{ title_group.country_from }}
          </div>
          <div v-if="title_group.series.id" class="information-line series">
            <span class="item-title">Series:</span>
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
    <div class="actions">
      <div>
        <i
          v-if="title_group.is_subscribed"
          v-tooltip.top="'Unsubscribe'"
          @click="unsubscribe"
          class="pi pi-bell-slash"
        />
        <i v-else v-tooltip.top="'Subscribe'" @click="subscribe" class="pi pi-bell" />
        <i v-tooltip.top="'Bookmark'" class="pi pi-bookmark" />
      </div>
      <div>
        <i v-tooltip.top="'Edit'" class="pi pi-pen-to-square" />
        <i @click="uploadTorrent" v-tooltip.top="'Upload Torrent'" class="pi pi-upload" />
        <i v-tooltip.top="'Request format'" class="pi pi-shopping-cart" />
      </div>
      <FloatLabel class="sort-by-select" variant="on">
        <Select
          v-model="sortBy"
          inputId="sort_by"
          :options="selectableSortingOptions"
          class="select"
          size="small"
        />
        <label for="sort_by">Sort by</label>
      </FloatLabel>
    </div>
    <TitleGroupTable :title_group="title_group" />
    <Accordion
      v-if="title_group.torrent_requests.length != 0"
      value="0"
      class="torrent-requests dense-accordion"
    >
      <AccordionPanel value="0">
        <AccordionHeader>Requests ({{ title_group.torrent_requests.length }})</AccordionHeader>
        <AccordionContent>
          <TorrentRequestsTable
            :torrent_requests="title_group.torrent_requests"
            :title_group="title_group"
          />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <ContentContainer class="description" v-if="title_group">
      <span class="title">Description</span>
      <div class="title-group-description">
        {{ title_group.description }}
      </div>
      <div v-for="edition_group in title_group.edition_groups" :key="edition_group.id">
        <div v-if="edition_group.description" class="edition-description">
          <div class="edition-group-slug">{{ getEditionGroupSlug(edition_group) }}</div>
          {{ edition_group.description }}
        </div>
      </div>
    </ContentContainer>
    <GeneralComments :comments="title_group.title_group_comments" />
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
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import { subscribeToItem, unsubscribeToItem } from '@/services/api/generalService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { getEditionGroupSlug } from '@/services/helpers'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'

export default {
  components: {
    ContentContainer,
    TitleGroupTable,
    Galleria,
    Image,
    FloatLabel,
    Select,
    AffiliatedArtist,
    ExternalLink,
    GeneralComments,
    TorrentRequestsTable,
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionPanel,
  },
  setup() {},
  data() {
    return {
      title_group: null,
      subscription_loading: false,
      sortBy: 'Edition',
      selectableSortingOptions: ['Edition', 'Size', 'Seeders', 'Completed', 'Uploaded at'],
    }
  },
  created() {
    getTitleGroup(this.$route.query.id?.toString()).then((data) => {
      this.title_group = data
    })
  },
  computed: {
    getEditionGroupSlug() {
      return (edition_group) => {
        return getEditionGroupSlug(edition_group)
      }
    },
  },
  methods: {
    uploadTorrent() {
      const titleGroupStore = useTitleGroupStore()
      titleGroupStore.id = this.title_group.id
      titleGroupStore.edition_groups = this.title_group.edition_groups
      titleGroupStore.content_type = this.title_group.content_type
      this.$router.push({ path: '/upload' })
    },
    subscribe() {
      subscribeToItem(this.$route.query.id, 'title_group').then(() => {
        this.title_group.is_subscribed = true
      })
    },
    unsubscribe() {
      unsubscribeToItem(this.$route.query.id, 'title_group').then(() => {
        this.title_group.is_subscribed = false
      })
    },
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
.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
.actions i {
  margin: 0px 0.5em;
  color: white;
  cursor: pointer;
}
.sort-by-select {
}
.torrent-requests {
  margin-top: 20px;
}
.description {
  margin-top: 20px;
}
.description .title {
  font-size: 1.2em;
}
.title-group-description {
  margin-top: 10px;
  margin-bottom: 25px;
}
.edition-description {
  margin-top: 15px;
}
.edition-description .edition-group-slug {
  color: var(--color-primary);
  margin-bottom: 5px;
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
