<template>
  <!-- TODO: use skeletons while the data is loading -->
  <div v-if="title_group">
    <TitleGroupHeader :title_group />
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
import TitleGroupTable from '@/components/title_group/TitleGroupTable.vue'
import TorrentRequestsTable from '@/components/request/TorrentRequestsTable.vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import TitleGroupHeader from '@/components/title_group/TitleGroupHeader.vue'
import { subscribeToItem, unsubscribeToItem } from '@/services/api/generalService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { getEditionGroupSlug } from '@/services/helpers'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'

export default {
  components: {
    ContentContainer,
    TitleGroupTable,
    FloatLabel,
    Select,
    GeneralComments,
    TorrentRequestsTable,
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionPanel,
    TitleGroupHeader,
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
