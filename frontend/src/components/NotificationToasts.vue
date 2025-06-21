<template>
  <Toast v-for="type in notificationTypes" :key="type.name" position="bottom-right" :group="type.name">
    <template #message="slotProps">
      <div class="p-toast-detail notification">
        {{ slotProps.message.detail }}
        <br />
        <RouterLink :to="type.viewRoute">{{ t('general.view') }}</RouterLink>
      </div>
    </template>
  </Toast>
</template>

<script setup lang="ts">
import { removeToastGroup, showToast } from '@/main'
import { useNotificationsStore } from '@/stores/notifications'
import { Toast } from 'primevue'
import { nextTick } from 'vue'
import { ref } from 'vue'
import { watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'

const notificationsStore = useNotificationsStore()
const { t } = useI18n()

const notificationTypes = ref([{ name: 'conversation', viewRoute: '/conversations' }])

watch(
  () => notificationsStore.unread_conversations_amount,
  async (newValue) => {
    removeToastGroup('conversation')
    if (newValue > 0) {
      await nextTick()
      showToast('', t('user.unread_messages_in_conversation', [newValue]), 'info', undefined, false, 'conversation')
    }
  },
  { immediate: true },
)
</script>
<style scoped>
.notification {
  margin-bottom: -3px;
}
</style>
