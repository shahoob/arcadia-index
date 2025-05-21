<template>
  <div class="menu-bar">
    <RouterLink v-for="item in menuItems" :key="item.label" class="item" :to="item.route">
      <Button severity="secondary" :label="item.label" size="small" />
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from 'primevue'

const menuItems = ref([
  { label: 'Torrents', route: '/torrents' },
  { label: 'Collages', route: '' },
  { label: 'Requests', route: '' },
  { label: 'Forum', route: '/forum' },
  { label: 'IRC', route: '' },
  { label: 'Top', route: '' },
  { label: 'Rules', route: '' },
  { label: 'Wiki', route: '' },
])
</script>

<style scoped>
.menu-bar {
  display: flex;
  justify-content: center;
}
.item {
  margin: 0px 7px;
}
</style>
