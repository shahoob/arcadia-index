<template>
  <div v-if="wikiArticle" class="wiki-article">
    <ContentContainer :containerTitle="wikiArticle.title">
      <BBCodeRenderer :content="wikiArticle.body" />
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import { getWikiArticle, type WikiArticle } from '@/services/api/wikiService'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const wikiArticle = ref<WikiArticle>()

const fetchWikiArticle = async (articleId: number) => {
  getWikiArticle(articleId).then((article) => {
    wikiArticle.value = article
  })
}

onMounted(() => {
  fetchWikiArticle(parseInt(route.params.id as string))
})
</script>

<style>
.wiki-article {
  margin-bottom: 20px;
}
</style>
