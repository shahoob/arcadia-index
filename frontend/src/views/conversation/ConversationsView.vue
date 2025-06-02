<template>
  <div class="title">{{ t('conversation.conversation', 2) }}</div>
  {{ conversations }}
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { onMounted } from 'vue'
import { getConversations, type ConversationsOverview } from '@/services/api/conversationService'
import { ref } from 'vue'

const { t } = useI18n()
const conversations = ref<ConversationsOverview[]>()

const fetchConversations = async () => {
  getConversations().then((overview) => {
    conversations.value = overview.conversations
  })
}

onMounted(() => {
  fetchConversations()
})
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.conversation-subject {
  .p-inputtext {
    width: 100%;
  }
}
.bbcode-editor {
  margin-top: 15px;
}
</style>
