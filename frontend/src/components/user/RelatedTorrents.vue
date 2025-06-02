<template>
  <ContentContainer
    :containerTitle="t(`user.${props.type}`)"
    :containerTitleLink="`/torrents?${props.type === 'uploads' ? 'created_by_id' : 'snatched_by_id'}=${userId}`"
  >
    <div class="last-uploads" v-if="titleGroups">
      <TitleGroupPreviewCoverOnly
        v-for="titleGroup in titleGroups"
        :key="titleGroup.id"
        :titleGroup="titleGroup"
      />
    </div>
    <div v-else>
      {{ t(`user.no_${props.type}_explanation`) }}
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import type { TitleGroupHierarchyLite } from '@/services/api/artistService'
import ContentContainer from '../ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '../title_group/TitleGroupPreviewCoverOnly.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  titleGroups: TitleGroupHierarchyLite[]
  userId: number
  type: 'snatches' | 'uploads'
}>()
</script>

<style>
.last-uploads {
  display: flex;
  overflow-y: scroll;
  > * {
    margin-right: 10px;
  }
}
</style>
