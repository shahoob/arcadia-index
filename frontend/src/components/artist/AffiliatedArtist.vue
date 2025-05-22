<template>
  <div class="affiliated-artist">
    <Image :src="affiliated_artist.artist.pictures[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <RouterLink :to="`/artist/${affiliated_artist.artist.id}`">
      <div class="name">{{ affiliated_artist.artist.name }}</div>
    </RouterLink>
    <span class="nickname" v-if="affiliated_artist.nickname">
      ({{ affiliated_artist.nickname }})
    </span>
    <div class="roles">
      <template v-for="(role, i) in affiliated_artist.roles" :key="role">
        <span class="role">{{ t(`artist.role.${role}`) }}</span>
        <br v-if="(i + 1) % 2 === 0 && i < affiliated_artist.roles.length - 1" />
        <template v-else-if="i < affiliated_artist.roles.length - 1"> , </template>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AffiliatedArtistHierarchy } from '@/services/api/artistService'
import { Image } from 'primevue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  affiliated_artist: AffiliatedArtistHierarchy
}>()
</script>
<style scoped>
.affiliated-artist {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.roles {
  font-size: 0.8em;
  text-align: center;
}
.nickname {
  font-size: 0.8em;
}
.role {
  font-weight: bold;
  color: var(--color-primary);
}
</style>

<style>
.affiliated-artist img {
  height: 10em !important;
  border-radius: 7px;
}
</style>
