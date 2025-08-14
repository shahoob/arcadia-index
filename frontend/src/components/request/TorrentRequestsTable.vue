<template>
  <DataTable :value="torrentRequests" size="small">
    <Column header="File">
      <template #body="slotProps">
        <TorrentRequestFileSlug :torrentRequest="slotProps.data" :contentType />
      </template>
    </Column>
    <Column header="Edition">
      <template #body="slotProps">
        <TorrentRequestEditionSlug :torrentRequest="slotProps.data" :contentType />
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
import type { TorrentRequestHierarchyLite } from '@/services/api/torrentRequestService'
import TorrentRequestFileSlug from './TorrentRequestFileSlug.vue'
import TorrentRequestEditionSlug from './TorrentRequestEditionSlug.vue'
import type { ContentType } from '@/services/api/torrentService'
import { bytesToReadable } from '@/services/helpers'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[]
  contentType: ContentType
}>()
</script>
<style scoped></style>
