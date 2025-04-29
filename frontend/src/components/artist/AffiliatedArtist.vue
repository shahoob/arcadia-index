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
    <div class="roles">
      <span v-for="(role, i) in affiliated_artist.roles" :key="role">
        <span v-if="i != 0">, </span><span class="role">{{ $t(`artist.role.${role}`) }} </span>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AffiliatedArtistHierarchy } from '@/services/api/artistService'
import { Image } from 'primevue'

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
