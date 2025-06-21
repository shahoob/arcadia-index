<template>
  <div class="title">{{ t('conversation.conversation', 2) }}</div>
  <div v-if="conversations">
    <div class="card">
      <DataTable :value="conversations" :rowClass="(conversation) => (isConversationRead(conversation) ? '' : 'bg-unread')">
        <Column :header="t('conversation.subject')">
          <template #body="slotProps">
            <RouterLink :to="`/conversation/${slotProps.data.id}`">
              {{ slotProps.data.subject }}
            </RouterLink>
          </template>
        </Column>
        <Column :header="t('conversation.last_message')">
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.last_message.created_at) }}
            {{ t('general.by') }}
            <RouterLink :to="`/user/${slotProps.data.last_message.created_by.id}`">
              {{ slotProps.data.last_message.created_by.username }}
            </RouterLink>
          </template>
        </Column>
        <Column :header="t('general.started')">
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.created_at) }}
          </template>
        </Column>
        <Column :header="t('conversation.correspondant')">
          <template #body="slotProps">
            <RouterLink :to="`/user/${slotProps.data.correspondant.id}`">
              {{ slotProps.data.correspondant.username }}
            </RouterLink>
          </template>
        </Column>
      </DataTable>
    </div>
  </div>
</template>

<script setup lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { useI18n } from 'vue-i18n'
import { onMounted } from 'vue'
import { getConversations, type ConversationOverview } from '@/services/api/conversationService'
import { ref } from 'vue'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const conversations = ref<ConversationOverview[]>()

const fetchConversations = async () => {
  getConversations().then((overview) => {
    conversations.value = overview.conversations
  })
}

const isConversationRead = (c: ConversationOverview) => {
  const userId = useUserStore().id
  return (
    c.last_message.created_by.id === userId ||
    (userId === c.receiver_id
      ? c.receiver_last_seen_at !== null && new Date(c.receiver_last_seen_at).getTime() > new Date(c.last_message.created_at).getTime()
      : new Date(c.sender_last_seen_at).getTime() > new Date(c.last_message.created_at).getTime())
  )
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
