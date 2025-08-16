<template>
  <RouterLink :to="`/user/${user.id}`">
    {{ user.username }}
  </RouterLink>
  <i v-if="user.banned" v-tooltip.top="t('user.banned')" class="banned pi pi-ban" />
  <i v-if="!user.banned && user.warned" v-tooltip.top="t('user.warned')" class="warned pi pi-exclamation-triangle" />
</template>

<script setup lang="ts">
import type { UserLite } from '@/services/api/userService'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  user: UserLite
}>()
</script>

<style scoped></style>
