<template>
  <ForumSearchForm />
  <DataTable v-if="search" :value="search">
    <Column field="name" :header="t('general.title')"> </Column>
    <Column field="latest_post" :header="t('forum.latest_post')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.latest_post.created_at) }} {{ t('general.by') }}
        <RouterLink :to="`/user/${slotProps.data.latest_post.created_by.id}`">
          {{ slotProps.data.latest_post.created_by.username }}
        </RouterLink>
      </template>
    </Column>
    <Column field="posts_amount" :header="t('forum.posts')" />
  </DataTable>
</template>

<script setup lang="ts">
import { searchForumThreads } from '@/services/api/forumService'
import type { ForumThreadHierarchy } from '@/services/api/forumService'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'
import { useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { ref } from 'vue'
import ForumSearchForm from '@/components/forum/ForumSearchForm.vue'

const { t } = useI18n()
const route = useRoute()

const search = ref<ForumThreadHierarchy[]>()

onMounted(() => {
  searchForumThreads({ title: route.query.title as string }).then((v) => {
    search.value = v
  })
})
</script>

<style scoped>
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}
</style>
