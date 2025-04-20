<template>
  <DataTable
    v-model:expandedRows="expandedRows"
    :value="title_group.edition_groups.flatMap((edition_group: Object) => edition_group.torrents)"
    rowGroupMode="subheader"
    :groupRowsBy="isGrouped ? 'edition_group_id' : null"
    sortMode="single"
    :sortField="sortBy == 'edition' ? '' : sortBy"
    :sortOrder="1"
    tableStyle="min-width: 50rem"
    size="small"
    :pt="{ rowGroupHeaderCell: { colspan: 8 } }"
  >
    <Column expander style="width: 1em" v-if="!preview" />
    <Column style="width: 1em" v-else />
    <Column header="Properties" style="min-width: 300px">
      <template #body="slotProps">
        <a
          :href="
            preview ? `/title-group?id=${title_group.id}&torrentId=${slotProps.data.id}` : null
          "
          @click="preview ? null : toggleRow(slotProps.data)"
          class="cursor-pointer"
        >
          <span v-if="slotProps.data.container && title_group.content_type != 'Music'">{{
            slotProps.data.container
          }}</span>
          <span v-if="slotProps.data.video_codec"> / {{ slotProps.data.video_codec }}</span>
          <span v-if="slotProps.data.video_resolution">
            / {{ slotProps.data.video_resolution }}</span
          >
          <span v-if="slotProps.data.audio_codec">
            <span v-if="title_group.content_type != 'Music'">/ </span
            >{{ slotProps.data.audio_codec }}</span
          >
          <span v-if="slotProps.data.audio_channels"> / {{ slotProps.data.audio_channels }}</span>
          <span v-if="slotProps.data.audio_bitrate_sampling">
            / {{ slotProps.data.audio_bitrate_sampling }}</span
          >
          <span
            v-if="
              slotProps.data.languages.length === 1 && slotProps.data.languages[0] !== 'English'
            "
          >
            / {{ slotProps.data.languages[0] }}
          </span>
          <span v-if="slotProps.data.languages.length > 1"> / Multi-Language </span>
          <span v-for="(feature, index) in slotProps.data.features" :key="index">
            / <span class="bold">{{ feature }}</span>
          </span>
          <span v-if="slotProps.data.release_group"> / {{ slotProps.data.release_group }}</span>
          <span v-if="slotProps.data.trumpable != ''">
            / <span class="warning">Trumpable</span>
          </span>
          <span v-if="slotProps.data.reports"> / <span class="danger">Reported</span> </span>
        </a>
      </template>
    </Column>
    <Column header="Uploaded">
      <template #body="slotProps">
        {{ $timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column header="">
      <template #body="slotProps">
        <i
          v-tooltip.top="'Download'"
          class="action pi pi-download"
          @click="downloadTorrent(slotProps.data.id)"
        />
        <i
          v-tooltip.top="'Report'"
          class="action pi pi-flag"
          @click="reportTorrent(slotProps.data.id)"
        />
        <i v-tooltip.top="'Copy permalink'" class="action pi pi-link" />
        <i v-tooltip.top="'Edit'" class="action pi pi-pen-to-square" />
      </template>
    </Column>
    <Column header="Size">
      <template #body="slotProps"> {{ $bytesToReadable(slotProps.data.size) }} </template>
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
    <template #groupheader="slotProps" v-if="isGrouped">
      <div class="edition-group-header">
        {{ getEditionGroupSlug(slotProps.data.edition_group_id) }}
      </div>
    </template>
    <template #expansion="slotProps" v-if="!preview">
      <div class="pre-style release-name">{{ slotProps.data.release_name }}</div>
      <Accordion :value="[]" multiple class="dense-accordion">
        <AccordionPanel value="0" v-if="slotProps.data.reports">
          <AccordionHeader>Report information</AccordionHeader>
          <AccordionContent>
            <div class="report" v-for="report in slotProps.data.reports" :key="report.id">
              <span class="bold">{{ $timeAgo(report.reported_at) }}</span
              >: {{ report.description }}
            </div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="1">
          <AccordionHeader>Mediainfo</AccordionHeader>
          <AccordionContent>
            <pre class="mediainfo">{{ purifyHtml(slotProps.data.mediainfo) }}</pre>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.description" value="2">
          <AccordionHeader>Description</AccordionHeader>
          <AccordionContent>
            <div>{{ slotProps.data.description }}</div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.screenshots" value="3">
          <AccordionHeader>Screenshots</AccordionHeader>
          <AccordionContent>
            <div>{{ slotProps.data.screenshots }}</div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="4">
          <AccordionHeader>File List</AccordionHeader>
          <AccordionContent>
            <DataTable :value="slotProps.data.file_list.files" tableStyle="min-width: 50rem">
              <Column
                field="name"
                :header="(slotProps.data.file_list.parent_folder ?? '') + '/'"
              ></Column>
              <Column field="size" header="Size">
                <template #body="slotProps">
                  {{ $bytesToReadable(slotProps.data.size) }}
                </template>
              </Column>
            </DataTable>
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
    </template>
  </DataTable>
  <Dialog closeOnEscape modal header="Report torrent" v-model:visible="reportTorrentDialogVisible">
    <ReportTorrentDialog :torrentId="reportingTorrentId" @reported="torrentReported" />
  </Dialog>
</template>

<script lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import DOMPurify from 'dompurify'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import ReportTorrentDialog from '../torrent/ReportTorrentDialog.vue'
import Dialog from 'primevue/dialog'
import { downloadTorrent } from '@/services/api/torrentService'

export default {
  components: {
    DataTable,
    Column,
    AccordionPanel,
    AccordionHeader,
    AccordionContent,
    Accordion,
    ReportTorrentDialog,
    Dialog,
  },
  props: {
    title_group: {},
    preview: { default: false },
    sortBy: { default: 'edition' },
  },
  data() {
    return { expandedRows: [], reportTorrentDialogVisible: false, reportingTorrentId: 0 }
  },
  methods: {
    torrentReported(torrentReport) {
      this.reportTorrentDialogVisible = false
      const reportedTorrent = this.title_group.edition_groups
        .flatMap((edition_group) => edition_group.torrents)
        .find((torrent) => torrent.id == torrentReport.reported_torrent_id)
      if (reportedTorrent.reports) {
        reportedTorrent.reports.push(torrentReport)
      } else {
        reportedTorrent.reports = [torrentReport]
      }
    },
    reportTorrent(id: number) {
      this.reportingTorrentId = id
      this.reportTorrentDialogVisible = true
    },
    toggleRow(torrent) {
      if (!this.expandedRows.some((expandedTorrent) => expandedTorrent.id === torrent.id)) {
        this.expandedRows = [...this.expandedRows, torrent]
      } else {
        this.expandedRows = this.expandedRows.filter((t) => t.id !== torrent.id)
      }
    },
    purifyHtml(html: string) {
      return DOMPurify.sanitize(html)
    },
    downloadTorrent(torrentId: number) {
      downloadTorrent(torrentId)
    },
  },
  created() {
    if (this.$route.query.torrentId) {
      this.toggleRow(
        this.title_group.edition_groups
          .flatMap((edition_group) => edition_group.torrents)
          .find((torrent) => torrent.id == this.$route.query.torrentId),
      )
    }
  },
  computed: {
    isGrouped() {
      return this.sortBy === 'edition'
    },
    getEditionGroupSlug() {
      return (id: number) => {
        return this.$getEditionGroupSlug(
          this.title_group.edition_groups.find((group: object) => group.id === id),
        )
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
.release-name {
  margin-bottom: 10px;
  margin-left: 7px;
}
</style>
