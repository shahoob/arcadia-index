<template>
  <ContentContainer class="user-application">
    <div class="information">
      <div class="item">
        <span>{{ t('staff.user_application.body') }}</span>
        <div class="content">{{ userApplication.body }}</div>
      </div>
      <div class="item">
        <span>{{ t('user.email') }}</span>
        <div class="content">{{ userApplication.email }}</div>
      </div>
      <div class="item status">
        <span>{{ t('staff.user_application.status') }}</span>
        <div :class="`content ${userApplication.status}`">{{ userApplication.status }}</div>
      </div>
      <div class="item">
        <span>{{ t('general.sent') }}</span>
        <div class="content">{{ timeAgo(userApplication.created_at) }}</div>
      </div>
    </div>
    <div class="buttons" v-if="userApplication.status === 'pending'">
      <Button :label="t('user.send_invite')" size="small" severity="success" disabled v-tooltip.top="'Not implemented yet'" />
      <Button :label="t('staff.user_application.reject')" size="small" severity="danger" @click="updateApplication('rejected')" :loading="rejectLoading" />
    </div>
  </ContentContainer>
</template>
<script setup lang="ts">
import { updateUserApplication, type UserApplication, type UserApplicationStatus } from '@/services/api/userApplicationService'
import { useI18n } from 'vue-i18n'
import ContentContainer from '../ContentContainer.vue'
import { Button } from 'primevue'
import { timeAgo } from '@/services/helpers'
import { ref } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  userApplication: UserApplication
}>()

const emit = defineEmits<{
  applicationUpdated: [UserApplication]
}>()

const rejectLoading = ref(false)

const updateApplication = (status: UserApplicationStatus, invitation_id?: number) => {
  if (status === 'rejected') {
    rejectLoading.value = true
  }
  updateUserApplication({ status: status, invitation_id: invitation_id, user_application_id: props.userApplication.id })
    .then((updatedApplication) => {
      emit('applicationUpdated', updatedApplication)
    })
    .finally(() => {
      if (status === 'rejected') {
        rejectLoading.value = false
      }
    })
}
</script>

<style scoped>
.information {
  .item {
    margin-bottom: 20px;
    span {
      font-weight: bold;
    }
    .content {
      margin-top: 5px;
    }
  }
}
.status {
  .pending {
    color: #edc01e;
  }
  .accepted {
    color: green;
  }
  .rejected {
    color: red;
  }
}
.buttons {
  text-align: center;
  .p-button {
    margin: 0 5px;
  }
}
</style>
