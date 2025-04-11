<template>
  <div class="comments">
    <GeneralComment v-for="comment in comments" :key="comment.id" :comment="comment" />
  </div>
  <Form
    v-slot="$form"
    :initialValues="new_comment"
    :resolver
    @submit="onFormSubmit"
    validateOnSubmit
    :validateOnValueUpdate="false"
  >
    <div class="new-comment">
      <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
        {{ $form.content.error?.message }}
      </Message>
      <BBCodeEditor @value-change="newCommentUpdated" label="New comment" />
      <Button
        type="submit"
        label="Post"
        icon="pi pi-send"
        :loading="sending_comment"
        class="post-button"
      />
    </div>
  </Form>
</template>

<script lang="ts">
import GeneralComment from './GeneralComment.vue'
import { Button } from 'primevue'
import { postTitleGroupComment } from '@/services/api/commentService'
import BBCodeEditor from './BBCodeEditor.vue'
import { Form } from '@primevue/forms'
import Message from 'primevue/message'

export default {
  components: { GeneralComment, BBCodeEditor, Button, Form, Message },
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
    resolver({ values }) {
      const errors = {}

      if (values.content.length < 5) {
        console.log(values.content.length)
        errors.content = [{ message: 'You must write more than 5 characters' }]
      }

      return {
        errors,
      }
    },
    onFormSubmit({ valid }) {
      if (valid) {
        this.sendComment()
      }
    },
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
