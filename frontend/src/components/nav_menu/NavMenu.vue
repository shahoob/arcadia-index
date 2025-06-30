<template>
  <DesktopNavMenu />
  <MobileNavMenu />
</template>

<script setup lang="ts">
import DesktopNavMenu from './desktop/DesktopNavMenu.vue'
import MobileNavMenu from './mobile/MobileNavMenu.vue'
</script>

<style>
#desktopNavMenu {
  display: none;
}

@media screen and (min-width: 768px) {
  #mobileMenuTrigger {
    display: none;
  }

  #desktopNavMenu {
    display: block;
  }
}
</style>
