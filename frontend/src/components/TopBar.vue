<template>
  <div id="top-bar">
    <div class="left"></div>
    <div class="center">User Stats</div>
    <div class="right">
      <div class="actions">
        <a href="/upload"><Button icon="pi pi-upload" severity="secondary" size="small" /></a>
        <Button icon="pi pi-moon" @click="toggleDarkMode()" severity="secondary" size="small" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Button } from 'primevue'

export default {
  components: { Button },
  methods: {
    toggleDarkMode() {
      document.documentElement.classList.toggle('dark-theme')
    },
  },
}
</script>

<style scoped>
#top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--color-background-secondary);
  height: 45px;
  padding: 0 7px;
  width: 100%;
}
.actions .p-button {
  margin-left: 7px;
}
</style>
