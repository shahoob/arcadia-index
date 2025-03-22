<template>
  <DataTable
    v-model:expandedRows="expandedRows"
    :value="edition_groups.flatMap((edition_group: Object) => edition_group.torrents)"
    rowGroupMode="subheader"
    groupRowsBy="edition_group_id"
    sortMode="single"
    sortField="representative.name"
    :sortOrder="1"
    tableStyle="min-width: 50rem"
  >
    <Column expander style="width: 5rem" />
    <Column field="name" header="Name">
      <template #body="slotProps">
        <span v-if="slotProps.data.video_codec">{{ slotProps.data.video_codec }} / </span>
        {{ slotProps.data.container }} /
        <span v-if="slotProps.data.video_resolution">{{ slotProps.data.video_resolution }} / </span>
        <span v-if="slotProps.data.audio_codec">{{ slotProps.data.audio_codec }} / </span>
        <span v-for="feature in slotProps.data.features" :key="feature">
          <span class="feature">{{ feature }} / </span></span
        >
        {{ slotProps.data.release_group ? slotProps.data.release_group : 'NoGrp' }}
      </template>
    </Column>
    <Column field="date" header="Uploaded" style="min-width: 200px">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column field="actions" header="" style="min-width: 200px">
      <template #body="slotProps">
        <i class="action pi pi-download" /> <i class="action pi pi-flag" />
        <i class="action pi pi-link"
      /></template>
    </Column>
    <Column field="size" header="Size" style="min-width: 200px">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.size) }}
      </template>
    </Column>
    <template #groupheader="slotProps">
      {{ new Date(getEditionGroup(slotProps.data.edition_group_id).release_date).getFullYear() }} -
      {{ getEditionGroup(slotProps.data.edition_group_id).name }} /
      {{ getEditionGroup(slotProps.data.edition_group_id).source }}
    </template>
    <template #expansion="slotProps">
      <pre class="mediainfo">{{ purifyHtml(slotProps.data.mediainfo) }}</pre>
    </template>
  </DataTable>
</template>

<script lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo, bytesToReadable } from '@/services/helpers'
import DOMPurify from 'dompurify'

export default {
  components: { DataTable, Column },
  props: {
    edition_groups: [],
  },
  data() {
    return { expandedRows: [] }
  },
  methods: {
    timeAgo(date: string) {
      return timeAgo(date)
    },
    bytesToReadable(bytes: Number) {
      return bytesToReadable(bytes)
    },
    purifyHtml(html: string) {
      return DOMPurify.sanitize(html)
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
.action {
  margin-right: 7px;
}
.mediainfo {
  border: 2px dotted black;
  padding: 5px;
}
</style>
