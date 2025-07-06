<template>
  <div id="home-page">
    <div class="main">
      <div class="announcements">
        <AnnouncementComponent v-for="announcement in recentAnnouncements" :key="announcement.id" :announcement class="announcement" />
      </div>
    </div>
    <div class="sidebar">
      <ContentContainer :containerTitle="t('statistics.statistics')" v-if="stats">
        <div v-for="(value, statName) in stats" :key="statName">
          {{ t(`statistics.${statName}`) }}: {{ value }} <span v-if="statName.includes('users_active')">({{ (value / stats.enabled_users) * 100 }}%)</span>
        </div>
      </ContentContainer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getHome, type ForumPostAndThreadName, type HomeStats } from '@/services/api/homeService'
import { onMounted } from 'vue'
import { ref } from 'vue'
import AnnouncementComponent from '@/components/forum/AnnouncementComponent.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const recentAnnouncements = ref<ForumPostAndThreadName[]>()
const stats = ref<HomeStats>()

const fetchHome = async () => {
  getHome().then((data) => {
    recentAnnouncements.value = data.recent_announcements
    stats.value = data.stats
  })
}

onMounted(() => {
  fetchHome()
})
</script>

<style>
#home-page {
  display: flex;
  justify-content: space-between;
}
.main {
  width: 77%;
}
.sidebar {
  width: 22%;
}
.announcement {
  margin-bottom: 10px;
}
</style>
