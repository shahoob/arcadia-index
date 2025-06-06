<template>
  <DataTable :value="torrentRequests" size="small">
    <Column header="Requirements">
      <template #body="slotProps">
        <TorrentSlug :torrent="slotProps.data" :contentType />
      </template>
    </Column>
    <Column field="edition_name" header="Edition"></Column>
    <Column header="Bounty">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.bounties.upload) }} +
        {{ slotProps.data.bounties.bonus_points }} bp
      </template>
    </Column>
    <Column header="Votes">
      <template #body="slotProps"> {{ slotProps.data.user_votes_amount }} </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import type { TorrentRequestHierarchyLite } from '@/services/api/torrentRequestService'
import TorrentSlug from '../torrent/TorrentSlug.vue'
import type { ContentType } from '@/services/api/torrentService'
import { bytesToReadable } from '@/services/helpers'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[]
  contentType: ContentType
}>()
</script>
<style scoped></style>
