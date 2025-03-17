<template>
  <Menubar :model="menuItems" class="p-menubar" />
</template>

<script lang="ts">
import { ref } from 'vue'
import Menubar from 'primevue/menubar'

export default {
  components: { Menubar },
  setup() {
    const menuItems = ref([
      { label: 'Torrents' },
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
}
</script>

<style scoped>
.p-menubar {
  background-color: #2c2c2c;
  color: white;
  border: none;
}
</style>
