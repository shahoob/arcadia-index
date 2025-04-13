<template>
  <DataTable
    v-model:expandedRows="expandedRows"
    :value="title_group.edition_groups.flatMap((edition_group: Object) => edition_group.torrents)"
    rowGroupMode="subheader"
    groupRowsBy="edition_group_id"
    sortMode="single"
    sortField="representative.name"
    :sortOrder="1"
    tableStyle="min-width: 50rem"
    size="small"
    :pt="{ rowGroupHeaderCell: { colspan: 8 } }"
  >
    <Column expander style="width: 1em" />
    <Column header="Properties" style="min-width: 300px">
      <template #body="slotProps">
        {{ getTorrentSlug(slotProps.data) }}
      </template>
    </Column>
    <Column header="Uploaded">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column header="">
      <template #body="slotProps">
        <i
          v-tooltip.top="'Download'"
          class="action pi pi-download"
          @click="downloadTorrent(slotProps.data.id)"
        />
        <i v-tooltip.top="'Report'" class="action pi pi-flag" />
        <i v-tooltip.top="'Copy permalink'" class="action pi pi-link" />
        <i v-tooltip.top="'Edit'" class="action pi pi-pen-to-square" />
      </template>
    </Column>
    <Column header="Size">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.size) }}
      </template>
    </Column>
    <!-- TODO: replace with real data from the tracker -->
    <Column style="width: 2.5em">
      <template #header><i class="pi pi-replay" v-tooltip.top="'Completed'" /></template>
      <template #body>10</template>
    </Column>
    <Column style="width: 2.5em">
      <template #header><i class="pi pi-arrow-up" v-tooltip.top="'Seeders'" /></template>
      <template #body><span style="color: green">5</span></template>
    </Column>
    <Column style="width: 2.5em">
      <template #header><i class="pi pi-arrow-down" v-tooltip.top="'Leechers'" /></template>
      <template #body>0</template>
    </Column>
    <template #groupheader="slotProps">
      <div class="edition-group-header">
        {{ getEditionGroupSlug(slotProps.data.edition_group_id) }}
      </div>
    </template>
    <template #expansion="slotProps" v-if="!preview">
      <Accordion :value="[]" multiple class="dense-accordion">
        <AccordionPanel value="0">
          <AccordionHeader>Mediainfo</AccordionHeader>
          <AccordionContent>
            <pre class="mediainfo">{{ purifyHtml(slotProps.data.mediainfo) }}</pre>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.description" value="1">
          <AccordionHeader>Description</AccordionHeader>
          <AccordionContent>
            <div>{{ slotProps.data.description }}</div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.screenshots" value="2">
          <AccordionHeader>Screenshots</AccordionHeader>
          <AccordionContent>
            <div>{{ slotProps.data.screenshots }}</div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="3">
          <AccordionHeader>File List</AccordionHeader>
          <AccordionContent>
            <DataTable :value="slotProps.data.file_list.files" tableStyle="min-width: 50rem">
              <Column
                field="name"
                :header="(slotProps.data.file_list.parent_folder ?? '') + '/'"
              ></Column>
              <Column field="size" header="Size">
                <template #body="slotProps">
                  {{ bytesToReadable(slotProps.data.size) }}
                </template>
              </Column>
            </DataTable>
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
    </template>
  </DataTable>
</template>

<script lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo, bytesToReadable, getEditionGroupSlug, getTorrentSlug } from '@/services/helpers'
import DOMPurify from 'dompurify'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import { downloadTorrent } from '@/services/api/torrentService'

export default {
  components: { DataTable, Column, AccordionPanel, AccordionHeader, AccordionContent, Accordion },
  props: {
    title_group: {},
    preview: { default: false },
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
    downloadTorrent(torrentId: Number) {
      downloadTorrent(torrentId)
    },
  },
  computed: {
    getEditionGroupSlug() {
      return (id: Number) => {
        return getEditionGroupSlug(
          this.title_group.edition_groups.find((group: Object) => group.id === id),
        )
      }
    },
    getTorrentSlug() {
      return (torrent) => {
        return getTorrentSlug(torrent)
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
  cursor: pointer;
}
.mediainfo {
  border: 2px dotted black;
  padding: 5px;
}
.edition-group-header {
  color: var(--color-primary);
}
.date {
  font-weight: bold;
}
</style>
