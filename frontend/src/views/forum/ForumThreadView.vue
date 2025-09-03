<template>
  <div v-if="forumThread">
    <div class="title">
      {{ forumThread.name }}
    </div>
    <GeneralComment v-for="post in forumThread.posts" :key="post.id" :comment="post" />
    <Form v-slot="$form" :initialValues="newPost" :resolver @submit="onFormSubmit" validateOnSubmit :validateOnValueUpdate="false">
      <div class="new-post">
        <BBCodeEditor
          :emptyInput="bbcodeEditorEmptyInput"
          @value-change="newPostUpdated"
          @input-emptied="bbcodeEditorEmptyInput = false"
          :label="t('forum.new_post')"
        >
          <template #buttons>
            <Button type="submit" label="Post" icon="pi pi-send" :loading="sendingPost" class="post-button" />
          </template>
        </BBCodeEditor>
        <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
          {{ $form.content.error?.message }}
        </Message>
      </div>
    </Form>
  </div>
</template>

<script setup lang="ts">
import { getForumThread, postForumPost, type UserCreatedForumPost, type ForumPostHierarchy, type ForumThreadAndPosts } from '@/services/api/forumService'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import GeneralComment from '@/components/community/GeneralComment.vue'
import type { FormSubmitEvent } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { Form } from '@primevue/forms'
import { Button } from 'primevue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'

const route = useRoute()
const { t } = useI18n()

const forumThread = ref<null | ForumThreadAndPosts>(null)
const newPost = ref<UserCreatedForumPost>({
  content: '',
  forum_thread_id: 0,
})
const sendingPost = ref(false)
const bbcodeEditorEmptyInput = ref(false)
const siteName = import.meta.env.VITE_SITE_NAME

onMounted(async () => {
  forumThread.value = await getForumThread(parseInt(route.params.id as string))

  document.title = forumThread.value ? `${forumThread.value.name} - ${siteName}` : `Forum thread - ${siteName}`
})

const resolver = () => {
  const errors: Partial<Record<keyof UserCreatedForumPost, { message: string }[]>> = {}

  if (newPost.value.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const onFormSubmit = ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendPost()
  }
}

const newPostUpdated = (content: string) => {
  newPost.value.content = content
}

const sendPost = async () => {
  if (!forumThread.value) {
    return
  }
  sendingPost.value = true
  newPost.value.forum_thread_id = parseInt(route.params.id as string)
  const createdPost: ForumPostHierarchy = {
    ...(await postForumPost(newPost.value)),
    created_by: useUserStore(),
  }
  newPost.value.content = ''
  forumThread.value.posts.push(createdPost)
  bbcodeEditorEmptyInput.value = true
  sendingPost.value = false
}
</script>

<style scoped>
.new-post {
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
