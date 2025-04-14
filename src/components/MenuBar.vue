<template>
  <div class="menu-bar">
    <a v-for="item in menuItems" :key="item.label" class="item" :href="item.route">
      <Button severity="secondary" :label="item.label" size="small" />
    </a>
  </div>
</template>

<script lang="ts">
import { ref } from 'vue'
import { Button } from 'primevue'

export default {
  components: { Button },
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
.menu-bar {
  display: flex;
  justify-content: center;
}
.item {
  margin: 0px 7px;
}
</style>
