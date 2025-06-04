<template>
  <span v-if="torrent.container && contentType != 'music'">{{ torrent.container }}</span>
  <span v-if="torrent.video_codec"> / {{ torrent.video_codec }}</span>
  <span v-if="torrent.video_resolution"> / {{ torrent.video_resolution }}</span>
  <span v-if="torrent.audio_codec">
    <span v-if="contentType != 'music'"> / </span>{{ torrent.audio_codec }}</span
  >
  <span v-if="torrent.audio_channels"> / {{ torrent.audio_channels }}</span>
  <span v-if="torrent.audio_bitrate_sampling"> / {{ torrent.audio_bitrate_sampling }}</span>
  <span v-if="torrent.languages.length === 1 && torrent.languages[0] !== 'English'">
    / {{ torrent.languages[0] }}
  </span>
  <span v-if="torrent.languages.length > 1"> / Multi-Language </span>
  <span v-for="(feature, index) in torrent.features" :key="index">
    / <span class="bold">{{ feature }}</span>
  </span>
  <span v-if="torrent.release_group"> / {{ torrent.release_group }}</span>
  <span v-if="torrent.trumpable != ''"> / <span class="warning">Trumpable</span> </span>
  <span v-if="torrent.reports.length !== 0"> / <span class="danger">Reported</span> </span>
</template>
<script lang="ts" setup>
import type { ContentType, TorrentHierarchyLite } from '@/services/api/torrentService'

defineProps<{
  torrent: TorrentHierarchyLite
  contentType: ContentType
}>()
</script>
