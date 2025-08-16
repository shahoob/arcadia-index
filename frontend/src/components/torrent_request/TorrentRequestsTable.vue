<template>
  <DataTable :value="torrentRequests" size="small">
    <Column header="Title" v-if="displayTitleGroup">
      <template #body="slotProps">
        <TitleGroupSlimHeader
          :titleGroup="slotProps.data.title_group"
          :series="slotProps.data.series"
          :nameLink="`/torrent-request/${slotProps.data.torrent_request.id}`"
        />
      </template>
    </Column>
    <Column header="Requirements">
      <template #body="slotProps">
        <RouterLink :to="`/torrent-request/${slotProps.data.torrent_request.id}`">
          <TorrentRequestSlug :torrentRequest="slotProps.data.torrent_request" :contentType="contentType ?? slotProps.data.title_group.content_type" />
        </RouterLink>
      </template>
    </Column>
    <Column header="Bounty">
      <template #body="slotProps"> {{ bytesToReadable(slotProps.data.bounty.upload) }} + {{ slotProps.data.bounty.bonus_points }} bp </template>
    </Column>
    <Column header="Votes">
      <template #body="slotProps"> {{ slotProps.data.user_votes_amount }} </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import type { TorrentRequestHierarchyLite, TorrentRequestWithTitleGroupLite } from '@/services/api/torrentRequestService'
import TorrentRequestSlug from './TorrentRequestSlug.vue'
import TitleGroupSlimHeader from '../title_group/TitleGroupSlimHeader.vue'
import { RouterLink } from 'vue-router'
import type { ContentType } from '@/services/api/torrentService'
import { bytesToReadable } from '@/services/helpers'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[] | TorrentRequestWithTitleGroupLite[]
  contentType?: ContentType
  displayTitleGroup?: boolean
}>()
</script>
<style scoped></style>
