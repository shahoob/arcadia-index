<template>
  <DataTable :value="torrentRequests" size="small">
    <Column header="Requirements">
      <!-- TODO: Add more details -->
      <template #body="slotProps">
        <span v-if="slotProps.data.container">{{ slotProps.data.container }}</span>
        <span v-else>{{ $t('torrent.any_container') }}</span>
        <span v-if="slotProps.data.video_codec"> / {{ slotProps.data.video_codec }} </span>
      </template>
    </Column>
    <Column field="edition_name" header="Edition"></Column>
    <Column header="Bounty">
      <template #body="slotProps">
        {{ $bytesToReadable(slotProps.data.bounty_upload) }} +
        {{ slotProps.data.bounty_bonus_points }} bp
      </template>
    </Column>
    <Column field="votes" header="Votes"></Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import type { TorrentRequest } from '@/services/api/torrentRequestService'

defineProps<{
  torrentRequests: TorrentRequest[]
}>()
</script>
<style scoped></style>
