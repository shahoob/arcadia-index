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
      <BBCodeEditor @value-change="newCommentUpdated" :label="t('community.new_comment')" />
      <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
        {{ $form.content.error?.message }}
      </Message>
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

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import GeneralComment from './GeneralComment.vue'
import { Button } from 'primevue'
import {
  postTitleGroupComment,
  type TitleGroupCommentHierarchy,
  type UserCreatedTitleGroupComment,
} from '@/services/api/commentService'
import { type TitleGroupHierarchy } from '@/services/api/torrentService'
import BBCodeEditor from './BBCodeEditor.vue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import Message from 'primevue/message'
import { useUserStore } from '@/stores/user'
import { useRoute } from 'vue-router'

defineProps<{
  comments: TitleGroupHierarchy[]
}>()

const { t } = useI18n()

const route = useRoute()

const new_comment = ref<UserCreatedTitleGroupComment>({
  content: '',
  refers_to_torrent_id: null,
  answers_to_comment_id: null,
  title_group_id: 0,
})
const sending_comment = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTitleGroupComment, { message: string }[]>> = {}

  if (values.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const onFormSubmit = ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendComment()
  }
}

const newCommentUpdated = (content: string) => {
  new_comment.value.content = content
}

const sendComment = async () => {
  sending_comment.value = true
  new_comment.value.title_group_id = parseInt(route.params.id as string)
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const created_comment: TitleGroupCommentHierarchy = {
    ...(await postTitleGroupComment(new_comment.value)),
    created_by: useUserStore(),
  }
  new_comment.value.content = ''
  new_comment.value.refers_to_torrent_id = null
  new_comment.value.answers_to_comment_id = null
  // TODO: use created_comment to update the comments list
  // this.comments.push(created_comment)
  sending_comment.value = false
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
