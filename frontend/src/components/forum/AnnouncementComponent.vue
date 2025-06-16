<template>
  <ContentContainer>
    <div class="top-line">
      <div class="left">
        <div class="title">{{ announcement.forum_thread_name }}</div>
        <RouterLink :to="`/forum/thread/${announcement.forum_thread_id}`" class="discuss">{{ t('forum.discuss') }}</RouterLink>
      </div>
      <span class="date">({{ timeAgo(announcement.created_at) }})</span>
    </div>
    <BBCodeRenderer :content="announcement.content" />
  </ContentContainer>
</template>

<script setup lang="ts">
import type { ForumPostAndThreadName } from '@/services/api/homeService'
import ContentContainer from '../ContentContainer.vue'
import { RouterLink } from 'vue-router'
import { timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import BBCodeRenderer from '../community/BBCodeRenderer.vue'

const { t } = useI18n()

defineProps<{
  announcement: ForumPostAndThreadName
}>()
</script>

<style scoped>
.top-line {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
  .left {
    display: flex;
    align-items: flex-end;
  }
  .discuss {
    margin-bottom: 5px;
  }
}
.title {
  margin-right: 5px;
}
</style>
