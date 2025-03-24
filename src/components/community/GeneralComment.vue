<template>
  <ContentContainer class="comment">
    <div>{{ comment.content }}</div>
  </ContentContainer>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
export default {
  components: { ContentContainer },
  props: {
    comment: {},
  },
}
</script>

<style scoped>
.comment {
  margin-top: 10px;
}
</style>
