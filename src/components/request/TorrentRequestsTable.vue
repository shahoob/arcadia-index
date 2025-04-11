<template>
  <DataTable :value="torrent_requests" size="small">
    <Column header="Requirements">
      <!-- TODO: Add more details -->
      <template #body="slotProps">
        <span v-if="slotProps.data.container">{{ slotProps.data.container }}</span>
        <span v-else>Any Container</span>
        <span v-if="slotProps.data.video_codec"> / {{ slotProps.data.video_codec }} </span>
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

<script lang="ts">
import { Column, DataTable } from 'primevue'
import { bytesToReadable } from '@/services/helpers'
export default {
  components: { DataTable, Column },
  props: {
    torrent_requests: {},
  },
  methods: {
    bytesToReadable(bytes: Number) {
      return bytesToReadable(bytes)
    },
  },
}
</script>
<style scoped></style>
