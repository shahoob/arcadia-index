<template>
  <div class="report-torrent">
    <FloatLabel>
      <Textarea class="description" name="description" v-model="report.description" rows="5" />
      <label for="description">{{ $t('general.description') }}</label>
    </FloatLabel>
    <Button label="Send report" size="small" :loading @click="sendReport()" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { reportTorrent, type TorrentReport } from '@/services/api/torrentService'
import { Textarea, FloatLabel } from 'primevue'
import Button from 'primevue/button'

export default defineComponent({
  components: { Textarea, FloatLabel, Button },
  props: { torrentId: {} },
  data() {
    return {
      report: { description: '', reported_torrent_id: 0 },
      loading: false,
    }
  },
  methods: {
    sendReport() {
      this.loading = true
      this.report.reported_torrent_id = this.torrentId
      reportTorrent(this.report).then((data: TorrentReport) => {
        this.loading = false
        this.$emit('reported', data)
      })
    },
  },
})
</script>

<style scoped>
.report-torrent {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.description {
  width: 25em;
  margin-bottom: 20px;
}
</style>
