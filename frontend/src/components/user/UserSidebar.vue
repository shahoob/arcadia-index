<template>
  <div id="user-sidebar">
    <Image
      :src="user.avatar ?? '/default_user_avatar.svg'"
      :alt="user.username + '\'s avatar'"
      preview
    >
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <ContentContainer :container-title="t('community.statistics')" class="stats-container">
      {{ t('user.joined_at') }}:
      <span v-tooltip.top="formatDate(user.created_at)">{{ timeAgo(user.created_at) }}</span>
      <br />
      {{ t('user.last_seen') }}:
      <span v-tooltip.top="formatDate(user.last_seen)">{{ timeAgo(user.last_seen) }}</span>
      <br />
      {{ t('user.class') }}: {{ user.class }}
      <br />
      {{ t('user.bonus_points') }}: {{ user.bonus_points }}
      <br />
      {{ t('general.uploaded') }}: {{ bytesToReadable(user.uploaded) }}
      <br />
      {{ t('general.uploaded_real') }}: {{ bytesToReadable(user.real_uploaded) }}
      <br />
      {{ t('general.downloaded') }}: {{ bytesToReadable(user.downloaded) }}
      <br />
      {{ t('general.downloaded_real') }}: {{ bytesToReadable(user.real_downloaded) }}
      <br />
      {{ t('user.seeding_size') }}: {{ bytesToReadable(user.seeding_size) }}
      <br />
    </ContentContainer>
    <ContentContainer :container-title="t('community.community')">
      {{ t('community.forum_threads') }}: {{ user.forum_threads }}
      <br />
      {{ t('community.forum_posts') }}: {{ user.forum_posts }}
      <br />
      {{ t('community.collages_started') }}: {{ user.collages_started }}
      <br />
      {{ t('community.torrent_comments') }}: {{ user.torrent_comments }}
      <br />
      {{ t('community.request_comments') }}: {{ user.request_comments }}
      <br />
      {{ t('community.request_voted') }}: {{ user.requests_voted }}
      <br />
      {{ t('community.request_filled') }}: {{ user.requests_filled }}
      <br />
      {{ t('community.artist_comments') }}: {{ user.artist_comments }}
      <br />
      {{ t('community.invited') }}: {{ user.invited }}
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import type { PublicUser, User } from '@/services/api/userService'
import { Image } from 'primevue'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { bytesToReadable, timeAgo, formatDate } from '@/services/helpers'

const { t } = useI18n()

defineProps<{
  user: User | PublicUser
}>()
</script>

<style scoped>
.stats-container {
  margin-top: 10px;
}
</style>
<style>
#user-sidebar {
  .p-image-preview {
    width: 100% !important;
    border-radius: 7px;
    img {
      width: 100% !important;
      border-radius: 7px;
    }
  }
}
</style>
