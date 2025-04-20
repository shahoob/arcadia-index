<template>
  <div class="affiliated-artist">
    <Image :src="affiliated_artist.artist.pictures[0]" preview>
      <template #previewicon>
        <i class="pi pi-search"></i>
      </template>
    </Image>
    <a :href="'/artist?id=' + affiliated_artist.artist.id">
      <div class="name">{{ affiliated_artist.artist.name }}</div>
    </a>
    <div class="roles">
      <span v-for="(role, i) in affiliated_artist.roles" :key="role">
        <span v-if="i != 0">, </span><span class="role">{{ $t(`artist.role.${role}`) }} </span>
      </span>
    </div>
  </div>
</template>

<script lang="ts">
import { Image } from 'primevue'
export default {
  components: { Image },
  props: {
    affiliated_artist: {},
  },
}
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
