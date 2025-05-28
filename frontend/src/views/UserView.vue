<template>
  <div id="user-view" v-if="user">
    <div class="main">
      <div class="top-bar">
        <div class="username">
          {{ user.username }}
          <i v-if="user.banned" v-tooltip.top="t('user.banned')" class="banned pi pi-ban" />
          <i
            v-if="!user.banned && user.warned"
            v-tooltip.top="t('user.warned')"
            class="warned pi pi-exclamation-triangle"
          />
        </div>
        <div v-if="userStore.class === 'staff'" class="actions">
          <i
            v-tooltip.top="t('user.warn')"
            class="cursor-pointer pi pi-exclamation-triangle"
            @click="warnUserDialogVisible = true"
          />
        </div>
      </div>
      <ContentContainer :containerTitle="t('general.description')" class="description">
        <BBCodeRenderer :content="user.description" />
      </ContentContainer>
      <div v-if="peers">
        <span class="bold">{{ t('torrent.clients_and_ips') }}</span>
        <PeerTable :peers class="peer-table" />
      </div>
    </div>
    <UserSidebar :user class="sidebar" />
  </div>
  <Dialog closeOnEscape modal :header="t('user.warn_user')" v-model:visible="warnUserDialogVisible">
    <WarnUserDialog @warned="warnUserDialogVisible = false" />
  </Dialog>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getMe, getUser, type Peer, type PublicUser, type User } from '@/services/api/userService'
import PeerTable from '@/components/user/PeerTable.vue'
import { useUserStore } from '@/stores/user'
import { useRoute } from 'vue-router'
import UserSidebar from '@/components/user/UserSidebar.vue'
import BBCodeRenderer from '@/components/BBCodeRenderer.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import WarnUserDialog from '@/components/user/WarnUserDialog.vue'
import { Dialog } from 'primevue'

const peers = ref<Peer[] | null>(null)
const user = ref<User | PublicUser | null>(null)

const userStore = useUserStore()
const route = useRoute()
const { t } = useI18n()

const warnUserDialogVisible = ref(false)

onMounted(async () => {
  if (parseInt(route.params.id.toString()) == userStore.id) {
    ;({ peers: peers.value, user: user.value } = await getMe())
    userStore.setUser(user.value as User)
  } else {
    ;({ user: user.value } = await getUser(parseInt(route.params.id.toString())))
  }
})
</script>

<style scoped>
#user-view {
  display: flex;
}
.main {
  width: 75%;
  margin-right: 10px;
}
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.username {
  font-weight: bold;
  font-size: 1.3em;
  margin-bottom: 10px;
  .banned {
    color: red;
  }
  .warned {
    color: yellow;
  }
}
.description {
  margin-bottom: 15px;
}
.peer-table {
  margin-top: 5px;
}
.sidebar {
  width: 25%;
}
</style>
