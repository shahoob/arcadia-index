<template>
  <div
    v-if="torrentRequestAndAssociatedData"
    id="title-group-view"
    :class="{
      'sidebar-right': userStore.settings.site_appearance.item_detail_layout == 'sidebar_right',
      'sidebar-left': userStore.settings.site_appearance.item_detail_layout == 'sidebar_left',
    }"
  >
    <div
      :class="{
        main: true,
        'with-sidebar': userStore.settings.site_appearance.item_detail_layout.includes('sidebar'),
      }"
    >
      <TitleGroupFullHeader
        :title_group="torrentRequestAndAssociatedData.title_group"
        :affiliatedArtists="torrentRequestAndAssociatedData.affiliated_artists"
        :series="torrentRequestAndAssociatedData.series"
        v-if="userStore.settings.site_appearance.item_detail_layout == 'header'"
      />
      <TitleGroupSlimHeader
        v-else
        :titleGroup="torrentRequestAndAssociatedData.title_group"
        :series="torrentRequestAndAssociatedData.series"
        class="slim-header title"
      />
      <div class="actions">
        <div>
          <i v-if="togglingSubscription" class="pi pi-hourglass" />
          <!-- <i
            v-else
            v-tooltip.top="t(`general.${titleGroupAndAssociatedData.is_subscribed ? 'un' : ''}subscribe`)"
            @click="toggleSubscribtion"
            :class="`pi pi-bell${titleGroupAndAssociatedData.is_subscribed ? '-slash' : ''}`"
          /> -->
          <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" />
        </div>
        <div>
          <!-- <i
            v-if="titleGroupAndAssociatedData.title_group.created_by_id === userStore.id || userStore.class === 'staff'"
            v-tooltip.top="t('general.edit')"
            class="pi pi-pen-to-square"
            @click="editTitleGroupDialogVisible = true"
          /> -->
          <i @click="uploadTorrent" v-tooltip.top="t('torrent.upload_torrent')" class="pi pi-upload" />
          <!-- <i @click="requestTorrent" v-tooltip.top="t('torrent.request_format')" class="pi pi-shopping-cart" /> -->
        </div>
      </div>
      <TorrentRequestDetails :torrentRequest="torrentRequestAndAssociatedData.torrent_request" />
      <!-- <ContentContainer :container-title="t('general.screenshots')" class="screenshots" v-if="titleGroupAndAssociatedData.title_group.screenshots.length !== 0">
        <CustomGalleria :images="titleGroupAndAssociatedData.title_group.screenshots" />
      </ContentContainer> -->
      <!-- <Accordion v-if="titleGroupAndAssociatedData.torrent_requests.length != 0" value="0" class="torrent-requests dense-accordion">
        <AccordionPanel value="0">
          <AccordionHeader> {{ t('torrent.requests') }} ({{ titleGroupAndAssociatedData.torrent_requests.length }}) </AccordionHeader>
          <AccordionContent>
            <TorrentRequestsTable
              :torrentRequests="titleGroupAndAssociatedData.torrent_requests"
              :contentType="titleGroupAndAssociatedData.title_group.content_type"
            />
          </AccordionContent>
        </AccordionPanel>
      </Accordion> -->
      <!-- <EmbeddedLinks
        class="embedded-links"
        v-if="Object.keys(titleGroupAndAssociatedData.title_group.embedded_links).length > 0"
        :links="titleGroupAndAssociatedData.title_group.embedded_links"
      /> -->
      <ContentContainer class="description" :container-title="t('title_group.description')">
        <div class="title-group-description">
          <BBCodeRenderer :content="torrentRequestAndAssociatedData.title_group.description" />
        </div>
      </ContentContainer>
      <!-- <TitleGroupComments :comments="titleGroupAndAssociatedData.title_group_comments" @newComment="newComment" /> -->
    </div>
    <div class="sidebar" v-if="userStore.settings.site_appearance.item_detail_layout.includes('sidebar')">
      <TitleGroupSidebar
        :title_group="torrentRequestAndAssociatedData.title_group"
        :affiliatedArtists="torrentRequestAndAssociatedData.affiliated_artists"
        :series="torrentRequestAndAssociatedData.series"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useUserStore } from '@/stores/user'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import TitleGroupSidebar from '@/components/title_group/TitleGroupSidebar.vue'
import ContentContainer from '@/components/ContentContainer.vue'
// import { getTitleGroup, type TitleGroup, type TitleGroupAndAssociatedData } from '@/services/api/torrentService'
// import TitleGroupTable from '@/components/title_group/TitleGroupTable.vue'
// import TorrentRequestsTable from '@/components/torrent_request/TorrentRequestsTable.vue'
// import Accordion from 'primevue/accordion'
// import AccordionPanel from 'primevue/accordionpanel'
// import AccordionHeader from 'primevue/accordionheader'
// import AccordionContent from 'primevue/accordioncontent'
import TitleGroupFullHeader from '@/components/title_group/TitleGroupFullHeader.vue'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
// import { subscribeToItem, unsubscribeToItem } from '@/services/api/generalService'
// import { useTitleGroupStore } from '@/stores/titleGroup'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
// import { showToast } from '@/main'
// import type { TitleGroupCommentHierarchy } from '@/services/api/commentService'
// import type { AffiliatedArtistHierarchy } from '@/services/api/artistService'
import { getTorrentRequest, type TorrentRequestAndAssociatedData } from '@/services/api/torrentRequestService'
import TorrentRequestDetails from '@/components/torrent_request/TorrentRequestDetails.vue'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const userStore = useUserStore()
// const titleGroupStore = useTitleGroupStore()

const torrentRequestAndAssociatedData = ref<TorrentRequestAndAssociatedData>()
const togglingSubscription = ref(false)
// const siteName = import.meta.env.VITE_SITE_NAME

onMounted(async () => {
  await fetchTorrentRequest()
})

const fetchTorrentRequest = async () => {
  torrentRequestAndAssociatedData.value = await getTorrentRequest(parseInt(route.params.id.toString()))
  console.log(torrentRequestAndAssociatedData.value)

  /*
    For series, the title group name just holds the season name (i.e. 'Season 1')
    so we want to show the series name itself in the document title as well.
  */
  // document.title = titleGroupAndAssociatedData.value.series.name
  //   ? `${titleGroupAndAssociatedData.value.title_group.name} (${titleGroupAndAssociatedData.value.series.name}) - ${siteName}`
  //   : `${titleGroupAndAssociatedData.value.title_group.name} - ${siteName}`
}

// const populateTitleGroupStore = () => {
//   if (titleGroupAndAssociatedData.value) {
//     titleGroupStore.id = titleGroupAndAssociatedData.value.title_group.id
//     titleGroupStore.original_release_date = titleGroupAndAssociatedData.value.title_group.original_release_date
//     titleGroupStore.name = titleGroupAndAssociatedData.value.title_group.name
//     titleGroupStore.edition_groups = titleGroupAndAssociatedData.value.edition_groups
//     titleGroupStore.content_type = titleGroupAndAssociatedData.value.title_group.content_type
//   }
// }

const uploadTorrent = () => {
  // populateTitleGroupStore()
  router.push({ path: '/upload' })
}

// const toggleSubscribtion = async () => {
//   if (titleGroupAndAssociatedData.value) {
//     togglingSubscription.value = true
//     if (titleGroupAndAssociatedData.value.is_subscribed) {
//       await unsubscribeToItem(parseInt(route.params.id.toString()), 'title_group')
//     } else {
//       await subscribeToItem(parseInt(route.params.id.toString()), 'title_group')
//     }
//     titleGroupAndAssociatedData.value.is_subscribed = !titleGroupAndAssociatedData.value.is_subscribed
//     showToast(
//       'Success',
//       t(`title_group.${titleGroupAndAssociatedData.value.is_subscribed ? 'subscription_successful' : 'unsubscription_successful'}`),
//       'success',
//       3000,
//     )
//     togglingSubscription.value = false
//   }
// }

// const newComment = (comment: TitleGroupCommentHierarchy) => {
//   titleGroupAndAssociatedData.value?.title_group_comments.push(comment)
// }

watch(() => route.params.id, fetchTorrentRequest, { immediate: true })
</script>

<style scoped>
.main.with-sidebar {
  width: 75%;
}
.sidebar {
  width: 25%;
}
.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}
.actions i {
  margin: 0px 0.5em;
  color: white;
  cursor: pointer;
}
.screenshots {
  margin-top: 20px;
}
.torrent-requests {
  margin-top: 20px;
}
.embedded-links {
  margin-top: 20px;
}
.description {
  margin-top: 20px;
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
.ratings {
  margin-top: 20px;
}
.comments {
  margin-top: 20px;
}
.edit-title-group {
  width: 60vw;
}
</style>
<style>
#title-group-view {
  .p-tabpanel {
    line-height: 0 !important;
  }
  .p-tab {
    padding: 10px !important;
  }
  .p-tabpanels {
    padding: 0 !important;
  }
}
</style>
