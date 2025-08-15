<template>
  <ContentContainer>
    <table>
      <tbody>
        <tr class="requirement" v-for="requirement in requirements" :key="requirement">
          <td class="name bold">{{ t(`torrent_request.acceptable_${requirement}`) }}</td>
          <td class="value">
            {{ typeof torrentRequest[requirement] === 'object' ? (torrentRequest[requirement] as string[]).join(', ') : torrentRequest[requirement] }}
          </td>
        </tr>
      </tbody>
    </table>
  </ContentContainer>
</template>

<script lang="ts" setup>
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'

const { t } = useI18n()
const requirements = ref<(keyof TorrentRequest)[]>([
  'audio_bitrate_sampling',
  'audio_channels',
  'audio_codec',
  'container',
  'edition_name',
  'features',
  'source',
  'video_codec',
  'video_resolution',
])

defineProps<{
  torrentRequest: TorrentRequest
}>()
</script>
<style scoped>
.requirement {
  .name {
    vertical-align: middle;
    text-align: right;
    padding-right: 5px;
  }
  .value {
    text-align: left;
  }
}
</style>
