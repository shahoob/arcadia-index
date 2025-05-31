<template>
  <ContentContainer
    :containerTitle="t('user.uploads')"
    :containerTitleLink="`/torrents?created_by_id=${userId}`"
  >
    <div class="last-uploads">
      <TitleGroupPreviewCoverOnly
        v-for="titleGroup in titleGroups"
        :key="titleGroup.id"
        :titleGroup="titleGroup"
      />
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import type { TitleGroupHierarchyLite } from '@/services/api/artistService'
import ContentContainer from '../ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '../title_group/TitleGroupPreviewCoverOnly.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  titleGroups: TitleGroupHierarchyLite[]
  userId: number
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
