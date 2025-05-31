<template>
  <div id="top-bar">
    <div class="left"></div>
    <div class="user-stats">
      <div class="stat" v-tooltip.bottom="'Uploaded'">
        <i class="pi pi-upload" />
        {{ bytesToReadable(user.uploaded) }}
      </div>
      <span class="stat" v-tooltip.bottom="'Downloaded'">
        <i class="pi pi-download" />{{ bytesToReadable(user.downloaded) }}
      </span>
      <span class="stat" v-tooltip.bottom="'Ratio'">
        <i class="pi pi-wave-pulse" />{{ user.ratio }}
      </span>
      <span class="stat" v-tooltip.bottom="'Bonus points'">
        <i class="pi pi-money-bill" />{{ user.bonus_points }}
      </span>
      <span class="stat" v-tooltip.bottom="'Freeleech tokens'">
        <i class="pi pi-ticket" />{{ user.freeleech_tokens }}
      </span>
    </div>
    <div class="right">
      <div class="actions">
        <RouterLink to="/upload">
          <Button icon="pi pi-upload" severity="secondary" size="small" />
        </RouterLink>
        <Button icon="pi pi-moon" @click="toggleDarkMode()" severity="secondary" size="small" />
        <RouterLink :onmouseenter="toggle" :to="`/user/${user.id}`">
          <Button icon="pi pi-user" severity="secondary" size="small" />
          <Popover ref="op">
            <div class="flex flex-col gap-4 w-[25rem]">
              <ul class="flex flex-col p-0 m-0 list-none">
                <li
                  class="flex gap-2 px-2 cursor-pointer list-item rounded-border hover:bg-emphasis"
                  @click="handleLogout"
                >
                  <i class="pi pi-sign-out small" />
                  <small class="font-medium">{{ t('general.logout') }}</small>
                </li>
              </ul>
            </div>
          </Popover>
        </RouterLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { Button } from 'primevue'
import { bytesToReadable } from '@/services/helpers'
import Popover from 'primevue/popover'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()

import { ref } from 'vue'
import router from '@/router'
const op = ref()

const user = useUserStore()

const toggle = (event: Event) => {
  op.value.toggle(event)
}

const handleLogout = () => {
  localStorage.removeItem('token')
  localStorage.removeItem('user')
  user.removeUser()
  router.push('/login')
}

const toggleDarkMode = () => {
  document.documentElement.classList.toggle('dark-theme')
}
</script>

<style scoped>
#top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--color-background-secondary);
  height: 45px;
  padding: 0 7px;
  width: 100%;
}

.list-item {
  align-items: center;
  transition: 0.3s ease;
}

.list-item:hover {
  color: var(--p-red-600);
}

.user-stats {
  font-size: 0.85em;
  display: flex;
  align-items: center;

  .stat {
    margin: 0px 10px;
    display: flex;
    align-items: center;
  }

  i {
    margin-right: 7px;
  }
}

.actions .p-button {
  margin-left: 7px;
}
</style>
