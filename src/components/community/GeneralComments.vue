<template>
  <div class="comments">
    <GeneralComment v-for="comment in comments" :key="comment.id" :comment="comment" />
  </div>
  <div class="new-comment">
    <BBCodeEditor @value-change="newCommentUpdated" label="New comment" />
    <Button
      type="button"
      label="Post"
      icon="pi pi-send"
      :loading="sending_comment"
      @click="sendComment"
      class="post-button"
    />
  </div>
</template>

<script lang="ts">
import GeneralComment from './GeneralComment.vue'
import { Button } from 'primevue'
import { postTitleGroupComment } from '@/services/api/commentService'
import BBCodeEditor from './BBCodeEditor.vue'
export default {
  // eslint-disable-next-line vue/no-reserved-component-names
  components: { GeneralComment, BBCodeEditor, Button },
  props: {
    comments: [],
  },
  data() {
    return {
      new_comment: {
        content: '',
        refers_to_torrent_id: null,
        answers_to_comment_id: null,
      },
      sending_comment: false,
    }
  },
  methods: {
    newCommentUpdated(content: string) {
      this.new_comment.content = content
    },
    sendComment() {
      this.sending_comment = true
      this.new_comment.title_group_id = parseInt(this.$route.query.id)
      postTitleGroupComment(this.new_comment).then((data) => {
        this.new_comment.content = ''
        this.new_comment.refers_to_torrent_id = null
        this.new_comment.answers_to_comment_id = null
        data.created_by = {}
        // TODO: don't mutate the prop
        this.comments.push(data)
        this.sending_comment = false
      })
    },
  },
}
</script>
<style scoped>
.new-comment {
  display: flex;
  flex-direction: column;
  margin-top: 30px;
  margin-bottom: 30px;
  align-items: flex-end;
}
.post-button {
  width: 5em;
  margin-top: 5px;
}
</style>
