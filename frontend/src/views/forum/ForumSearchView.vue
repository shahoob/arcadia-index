<template>
  <ForumSearchForm />
  <!-- <div class="top-bar"> -->
  <!--   <div class="title"> -->
  <!--     <RouterLink to="/forum">{{ search.category.name }}</RouterLink> > -->
  <!--     <RouterLink to="">{{ search.name }}</RouterLink> -->
  <!--   </div> -->
  <!--   <div class="actions"> -->
  <!--     <RouterLink :to="`/forum/thread/new?subCategoryId=${route.params.id}`"> -->
  <!--       <i v-tooltip.top="t('forum.new_thread')" class="cursor-pointer pi pi-plus" /> -->
  <!--     </RouterLink> -->
  <!--   </div> -->
  <!-- </div> -->
  <DataTable v-if="search" :value="search.posts">
    <!-- <Column field="forum" :header="t('general.name')"> -->
    <!--   <template #body="slotProps"> -->
    <!--     <RouterLink :to="`/forum/thread/${slotProps.data.id}`"> -->
    <!--       {{ slotProps.data.name }} -->
    <!--     </RouterLink> -->
    <!--   </template> -->
    <!-- </Column> -->
    <Column field="content" :header="t('general.content')">
      <!-- <template #body="slotProps"> -->
      <!--   {{ timeAgo(slotProps.data.latest_post.created_at) }} {{ t('general.by') }} -->
      <!--   <RouterLink :to="`/user/${slotProps.data.latest_post.created_by.id}`"> -->
      <!--     {{ slotProps.data.latest_post.created_by.username }} -->
      <!--   </RouterLink> -->
      <!-- </template> -->
    </Column>
    <!-- <Column field="posts_amount" :header="t('forum.posts')" /> -->
  </DataTable>
</template>

<script setup lang="ts">
import { getForumThreadAndPosts } from '@/services/api/forumService'
import type { ForumThreadAndPosts } from '@/services/api/forumService'
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

const search = ref<ForumThreadAndPosts>()

onMounted(() => {
  const isId = "id" in route.query
  console.log("query ", route.query)

  if (isId) {
    getForumThreadAndPosts({ id: route.query.id as unknown as number, }).then(v => {
      search.value = v
      console.log(search.value)
    })
  } else {
    getForumThreadAndPosts({ id: 1, }).then(v => {
      search.value = v
      console.log(search.value)
    })
  }
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
