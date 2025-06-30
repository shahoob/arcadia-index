<template>
  <div id="desktopNavMenu" class="actions">
    <RouterLink to="/upload">
      <Button icon="pi pi-upload" severity="secondary" size="small" />
    </RouterLink>
    <Button icon="pi pi-moon" @click="toggleDarkMode()" severity="secondary" size="small" />
    <RouterLink :to="`/user/${user.id}`">
      <Button :onmouseenter="show" :onmouseleave="onLeaveUserIcon" icon="pi pi-user" severity="secondary" size="small" />
      <Popover :onmouseleave="onLeavePopover" :onmouseenter="() => (isHoveringDropdown = true)" :dismissable="false" ref="op">
        <RouterLink to="/conversations">
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
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { Button } from 'primevue'
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
.actions .p-button {
  margin-left: 7px;
}
.user-action {
  padding: 5px 0;
  align-items: center;
  transition: 0.3s ease;
  &.sign-out {
    color: var(--p-red-600);
  }
}

#desktopNavMenu {
  display: none;
}

@media screen and (min-width: 768px) {
  #desktopNavMenu {
    display: block;
  }
}
</style>
