<template>
  <span v-for="(item, itemIndex) in computedSlug" :key="itemIndex">
    <span class="slash" v-if="itemIndex > 0"> / </span>
    <span>{{ item }}</span>
  </span>
</template>

<script lang="ts" setup>
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import type { ContentType } from '@/services/api/torrentService'
import { useI18n } from 'vue-i18n'
import { computed } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  torrentRequest: TorrentRequest
  contentType: ContentType
}>()

const computedSlug = computed<string[]>(() => {
  const slug: string[] = []

  function addIfPresent(arr: string[], value: string | string[] | undefined | null, specify_any: boolean, name?: string) {
    if (value && (typeof value === 'string' || value.length > 0)) {
      arr.push(Array.isArray(value) ? value.join(', ') : value)
    } else if (specify_any && value && typeof value === 'object' && value.length === 0) {
      arr.push(t(`edition_group.any_${name}`))
    }
  }

  addIfPresent(slug, props.torrentRequest.edition_name, false)
  addIfPresent(slug, props.torrentRequest.source, true, 'source')

  return slug
})
</script>
<style scoped>
.slash {
  font-weight: 300;
}
</style>
