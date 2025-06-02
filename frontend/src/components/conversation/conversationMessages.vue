<template>
  <div class="messages">
    <GeneralComment
      v-for="message in messages"
      :key="message.id"
      :comment="message"
      :reverseUserPosition="userStore.id === message.created_by.id"
      :class="`message ${userStore.id === message.created_by.id ? 'sent' : 'received'}`"
    />
  </div>
</template>

<script setup lang="ts">
import type { ConversationMessageHierarchy } from '@/services/api/conversationService'
import GeneralComment from '../community/GeneralComment.vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

defineProps<{
  messages: ConversationMessageHierarchy[]
}>()
</script>

<style>
.messages {
  display: flex;
  flex-direction: column;
}
.message {
  width: 95%;
  &.sent {
    align-self: flex-end;
  }
}
</style>
