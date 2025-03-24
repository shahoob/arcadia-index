<template>
  <div class="comments">
    <GeneralComment v-for="comment in comments" :key="comment.id" :comment="comment" />
  </div>
</template>

<script lang="ts">
import GeneralComment from './GeneralComment.vue'
export default {
  components: { GeneralComment },
  props: {
    comments: {},
  },
}
</script>
