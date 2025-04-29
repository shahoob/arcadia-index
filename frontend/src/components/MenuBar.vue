<template>
  <div class="menu-bar">
    <a v-for="item in menuItems" :key="item.label" class="item" :href="item.route">
      <Button severity="secondary" :label="item.label" size="small" />
    </a>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from 'primevue'

const menuItems = ref([
  { label: 'Torrents', route: '/torrents' },
  { label: 'Collages' },
  { label: 'Requests' },
  { label: 'Forums' },
  { label: 'IRC' },
  { label: 'Top 10' },
  { label: 'Rules' },
  { label: 'Wiki' },
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
