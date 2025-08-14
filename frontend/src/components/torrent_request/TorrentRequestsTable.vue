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
    <Column header="File">
      <template #body="slotProps">
        <TorrentRequestFileSlug
          :torrentRequest="slotProps.data.torrent_request ?? slotProps.data"
          :contentType="contentType ?? slotProps.data.title_group.content_type"
        />
      </template>
    </Column>
    <Column header="Edition">
      <template #body="slotProps">
        <TorrentRequestEditionSlug
          :torrentRequest="slotProps.data.torrent_request ?? slotProps.data"
          :contentType="contentType ?? slotProps.data.title_group.content_type"
        />
      </template>
    </Column>
    <Column header="Bounty">
      <template #body="slotProps"> {{ bytesToReadable(slotProps.data.bounties.upload) }} + {{ slotProps.data.bounties.bonus_points }} bp </template>
    </Column>
    <Column header="Votes">
      <template #body="slotProps"> {{ slotProps.data.user_votes_amount }} </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import type { TorrentRequestHierarchyLite, TorrentRequestWithTitleGroupLite } from '@/services/api/torrentRequestService'
import TorrentRequestFileSlug from './TorrentRequestFileSlug.vue'
import TorrentRequestEditionSlug from './TorrentRequestEditionSlug.vue'
import TitleGroupSlimHeader from '../title_group/TitleGroupSlimHeader.vue'
import type { ContentType } from '@/services/api/torrentService'
import { bytesToReadable } from '@/services/helpers'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[] | TorrentRequestWithTitleGroupLite[]
  contentType?: ContentType
  displayTitleGroup?: boolean
}>()
</script>
<style scoped></style>
