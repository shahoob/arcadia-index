<template>
  <div v-if="forumSubCategory">
    <div class="title">
      <RouterLink to="/forum">{{ forumSubCategory.category.name }}</RouterLink> >
      <RouterLink to="">{{ forumSubCategory.name }}</RouterLink>
    </div>
    <DataTable :value="forumSubCategory.threads">
      <Column field="name" :header="t('general.name')">
        <template #body="slotProps">
          <RouterLink :to="'/forum/thread/' + slotProps.data.id">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="latest_post" :header="t('forum.latest_post')">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.latest_post.created_at) }} {{ t('general.by') }}
          <RouterLink :to="'/user/' + slotProps.data.latest_post.created_by.id">
            {{ slotProps.data.latest_post.created_by.username }}
          </RouterLink>
        </template>
      </Column>
      <Column field="posts_amount" :header="t('forum.posts')" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { getForumSubCategory } from '@/services/api/forumService'
import type { ForumSubCategoryHierarchy } from '@/services/api/forumService'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'
import { useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { ref } from 'vue'

const { t } = useI18n()
const route = useRoute()

const forumSubCategory = ref<null | ForumSubCategoryHierarchy>(null)

onMounted(async () => {
  forumSubCategory.value = await getForumSubCategory(parseInt(route.params.id as string))
})
</script>

<style scoped>
.title {
  font-weight: bold;
  font-size: 2em;
}
</style>
