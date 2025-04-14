<template>
  <Menubar :model="menuItems" class="p-menubar">
    <template #item="slotProps">
      <a :href="slotProps.item.route">
        <Button severity="secondary" :label="slotProps.item.label" size="small" />
      </a>
    </template>
    <template #end>
      <Button icon="pi pi-moon" @click="toggleDarkMode()" />
    </template>
  </Menubar>
</template>

<script lang="ts">
import { ref } from 'vue'
import Menubar from 'primevue/menubar'
import { Button } from 'primevue'

export default {
  components: { Menubar, Button },
  setup() {
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

    return { menuItems }
  },
  methods: {
    toggleDarkMode() {
      document.documentElement.classList.toggle('dark-theme')
    },
  },
}
</script>

<style scoped>
.p-menubar {
  background-color: #181818;
  color: white;
  border: none;
}
</style>
