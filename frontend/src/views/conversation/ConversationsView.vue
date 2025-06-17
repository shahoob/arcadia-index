<template>
  <div class="title">{{ t('conversation.conversation', 2) }}</div>
  <div v-if="conversations">
    <div class="card">
      <DataTable :value="conversations">
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
import { getConversations, type ConversationsOverview } from '@/services/api/conversationService'
import { ref } from 'vue'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'

const { t } = useI18n()
const conversations = ref<ConversationsOverview[]>()
const siteName = import.meta.env.VITE_SITE_NAME

const fetchConversations = async () => {
  getConversations().then((overview) => {
    conversations.value = overview.conversations
  })
}

onMounted(() => {
  fetchConversations()
  document.title = `Conversations - ${siteName}`
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
