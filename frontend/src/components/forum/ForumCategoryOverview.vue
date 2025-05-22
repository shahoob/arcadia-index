<template>
  <div>
    <div class="title">{{ forumCategory.name }}</div>
    <DataTable :value="forumCategory.sub_categories">
      <Column field="name" :header="t('general.name')">
        <template #body="slotProps">
          <RouterLink :to="'/forum/sub-category/' + slotProps.data.id">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="latest_post_in_thread.name" :header="t('forum.latest_post')">
        <template #body="slotProps">
          <RouterLink :to="'/forum/thread/' + slotProps.data.latest_post_in_thread.id">
            {{ slotProps.data.latest_post_in_thread.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="latest_post_in_thread.created_at">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.latest_post_in_thread.created_at) }} {{ t('general.by') }}
          <RouterLink :to="'/user/' + slotProps.data.latest_post_in_thread.created_by.id">
            {{ slotProps.data.latest_post_in_thread.created_by.username }}
          </RouterLink>
        </template>
      </Column>
      <Column field="threads_amount" :header="t('forum.threads')" />
      <Column field="posts_amount" :header="t('forum.posts')" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import type { ForumCategoryHierarchy } from '@/services/api/forumService'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'

defineProps<{
  forumCategory: ForumCategoryHierarchy
}>()

const { t } = useI18n()
</script>

<style scoped>
.title {
  font-weight: bold;
  font-size: 2em;
  margin-bottom: 10px;
}
</style>
