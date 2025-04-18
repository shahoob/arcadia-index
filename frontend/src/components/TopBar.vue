<template>
  <div id="top-bar">
    <div class="left"></div>
    <div class="user-stats">
      <i v-tooltip.top="'Uploaded'" class="pi pi-upload" />{{ user.uploaded }}
    </div>
    <div class="right">
      <div class="actions">
        <a href="/upload"><Button icon="pi pi-upload" severity="secondary" size="small" /></a>
        <Button icon="pi pi-moon" @click="toggleDarkMode()" severity="secondary" size="small" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useUserStore } from '@/stores/user'
import { Button } from 'primevue'

export default {
  components: { Button },
  data() {
    return {
      user: {},
    }
  },
  methods: {
    toggleDarkMode() {
      document.documentElement.classList.toggle('dark-theme')
    },
  },
  created() {
    console.log(this.$bytesToReadable(654654654))
    this.user = useUserStore()
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
.user-stats {
  i {
    margin-right: 5px;
  }
}
.actions .p-button {
  margin-left: 7px;
}
</style>
