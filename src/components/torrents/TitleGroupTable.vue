<template>
  <DataTable
    :value="edition_groups.flatMap((edition_group: Object) => edition_group.torrents)"
    rowGroupMode="subheader"
    groupRowsBy="edition_group_id"
    sortMode="single"
    sortField="representative.name"
    :sortOrder="1"
    tableStyle="min-width: 50rem"
  >
    <Column field="name" header="Name" style="min-width: 200px">
      <template #body="slotProps">
        {{ slotProps.data.video_codec }} / {{ slotProps.data.audio_codec }}
        <span v-for="feature in slotProps.data.features" :key="feature">
          / <span class="feature">{{ feature }}</span></span
        >
        / {{ slotProps.data.release_group }}
      </template>
    </Column>
    <Column field="date" header="Uploaded" style="min-width: 200px">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column field="size" header="Size" style="min-width: 200px">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.size) }}
      </template>
    </Column>
    <template #groupheader="slotProps">
      {{ getEditionGroup(slotProps.data.edition_group_id).name }}
    </template>
  </DataTable>
</template>

<script lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo, bytesToReadable } from '@/services/helpers'

export default {
  components: { DataTable, Column },
  props: {
    edition_groups: [],
  },
  methods: {
    timeAgo(date: string) {
      return timeAgo(date)
    },
    bytesToReadable(bytes: Number) {
      return bytesToReadable(bytes)
    },
  },
  computed: {
    getEditionGroup() {
      return (id: Number) => {
        return this.edition_groups.find((group: Object) => group.id === id)
      }
    },
  },
}
</script>
<style scoped>
.feature {
  font-weight: bold;
}
</style>
