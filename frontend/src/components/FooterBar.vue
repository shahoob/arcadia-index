<template>
  <div id="footer">
    {{ t('footer.site_and_design') }}:
    <a href="https://github.com/Arcadia-Solutions/arcadia" target="_blank">Arcadia</a>
  </div>
</template>
<script lang="ts" setup>
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>
<style scoped>
#footer {
  text-align: center;
  margin-bottom: 10px;
}
</style>
