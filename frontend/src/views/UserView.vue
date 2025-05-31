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
      <ContentContainer :containerTitle="t('general.description')" class="section">
        <BBCodeRenderer :content="user.description" />
      </ContentContainer>
      <ContentContainer v-if="peers" :containerTitle="t('torrent.clients_and_ips')" class="section">
        <PeerTable :peers />
      </ContentContainer>
      <RelatedTorrents :titleGroups="uploadedTorrents" class="section" :userId="user.id" />
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
import RelatedTorrents from '@/components/user/RelatedTorrents.vue'
import { useUserStore } from '@/stores/user'
import { useRoute } from 'vue-router'
import UserSidebar from '@/components/user/UserSidebar.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import WarnUserDialog from '@/components/user/WarnUserDialog.vue'
import { Dialog } from 'primevue'
import type { TitleGroupHierarchyLite } from '@/services/api/artistService'

const peers = ref<Peer[] | null>(null)
const user = ref<User | PublicUser | null>(null)
const uploadedTorrents = ref<TitleGroupHierarchyLite[]>([])

const userStore = useUserStore()
const route = useRoute()
const { t } = useI18n()

const warnUserDialogVisible = ref(false)

onMounted(async () => {
  if (parseInt(route.params.id.toString()) == userStore.id) {
    ;({
      peers: peers.value,
      user: user.value,
      last_five_uploaded_torrents: uploadedTorrents.value,
    } = await getMe())
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
.section {
  margin-bottom: 15px;
}
.sidebar {
  width: 25%;
}
</style>
