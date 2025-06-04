<template>
  <span v-if="torrent.container && contentType != 'music'">{{ torrent.container }}</span>
  <span v-if="torrent.video_codec"> / {{ torrent.video_codec }}</span>
  <span v-if="torrent.video_resolution"> / {{ torrent.video_resolution }}</span>
  <span v-if="torrent.audio_codec">
    <span v-if="contentType != 'music'"> / </span>{{ torrent.audio_codec }}
  </span>
  <span v-if="torrent.audio_channels"> / {{ torrent.audio_channels }}</span>
  <span v-if="torrent.audio_bitrate_sampling"> / {{ torrent.audio_bitrate_sampling }}</span>
  <span v-if="torrent.languages.length === 1 && torrent.languages[0] !== 'English'">
    / {{ torrent.languages[0] }}
  </span>
  <span v-if="torrent.languages.length > 1"> / {{ t('torrent.multi_language') }} </span>
  <span v-for="(feature, index) in torrent.features" :key="index">
    / <span class="bold">{{ feature }}</span>
  </span>
  <span v-if="torrent.release_group"> / {{ torrent.release_group }}</span>
  <span v-if="'trumpable' in torrent && torrent.trumpable != ''">
    / <span class="warning">{{ t('torrent.trumpable') }}</span>
  </span>
  <span v-if="'reports' in torrent && torrent.reports.length !== 0">
    / <span class="danger">{{ t('general.reported') }}</span>
  </span>
</template>
<script lang="ts" setup>
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import type { ContentType, TorrentHierarchyLite } from '@/services/api/torrentService'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  torrent: TorrentHierarchyLite | TorrentRequest
  contentType: ContentType
}>()
</script>
