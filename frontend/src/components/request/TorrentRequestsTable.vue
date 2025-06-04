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
        {{ bytesToReadable(slotProps.data.bounty_upload) }} +
        {{ slotProps.data.bounty_bonus_points }} bp
      </template>
    </Column>
    <Column field="votes" header="Votes"></Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import TorrentSlug from '../torrent/TorrentSlug.vue'
import { bytesToReadable } from '@/services/helpers'
import type { ContentType } from '@/services/api/torrentService'

defineProps<{
  torrentRequests: TorrentRequest[]
  contentType: ContentType
}>()
</script>
<style scoped></style>
