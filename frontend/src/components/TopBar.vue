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
        <RouterLink :to="`/user/${user.id}`">
          <Button
            :onmouseenter="show"
            :onmouseleave="onLeaveUserIcon"
            icon="pi pi-user"
            severity="secondary"
            size="small"
          />
          <Popover
            :onmouseleave="onLeavePopover"
            :onmouseenter="() => (isHoveringDropdown = true)"
            :dismissable="false"
            ref="op"
          >
            <RouterLink to="/conversation">
              <div class="user-action flex gap-2 px-2 cursor-pointer">
                <i class="pi pi-envelope" />
                <small class="font-medium">{{ t('conversation.conversation', 2) }}</small>
              </div>
            </RouterLink>
            <div class="user-action sign-out flex gap-2 px-2 cursor-pointer" @click="handleLogout">
              <i class="pi pi-sign-out" />
              <small class="font-medium">{{ t('user.logout') }}</small>
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
const op = ref<InstanceType<typeof Popover> & HTMLAnchorElement>()

const user = useUserStore()

let isHoveringDropdown = false

const onLeavePopover = () => {
  isHoveringDropdown = false
  op.value?.hide()
}

const show = (event: Event) => {
  op.value?.show(event)
}

// delay a bit so we can know if the user unhovered the user icon,
// to hover the popover or not
const onLeaveUserIcon = () => {
  setTimeout(() => {
    if (!isHoveringDropdown) {
      op.value?.hide()
    }
  }, 100)
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
.user-action {
  padding: 5px 0;
  align-items: center;
  transition: 0.3s ease;
  &.sign-out:hover {
    color: var(--p-red-600);
  }
}
</style>
