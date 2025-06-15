<template>
  <TitleGroupSearchBar
    v-if="action === 'select'"
    class="name-input"
    :placeholder="t('general.name')"
    :clearInputOnSelect="false"
    @titleGroupSelected="titleGroupSelected"
    @createNew="createNew"
    :createOption="true"
    v-model="titleGroupName"
  />
  <CreateOrEditTitleGroup
    v-else-if="action === 'create'"
    @done="titleGroupCreated"
    :initial-title-group-name="titleGroupName"
    :initial-title-group-form="null"
  />
</template>

<script setup lang="ts">
import type { TitleGroup, TitleGroupLite } from '@/services/api/torrentService'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import CreateOrEditTitleGroup from './CreateOrEditTitleGroup.vue'
import TitleGroupSearchBar from './TitleGroupSearchBar.vue'
import { useTitleGroupStore } from '@/stores/titleGroup'

const { t } = useI18n()
const titleGroupStore = useTitleGroupStore()
const emit = defineEmits<{
  done: [titleGroup: TitleGroup | TitleGroupLite]
}>()

const action = ref<'select' | 'create'>('select')
const titleGroupName = ref('')

const titleGroupSelected = (titleGroup: TitleGroupLite) => {
  emit('done', titleGroup)
}
const titleGroupCreated = async (titleGroup: TitleGroup | TitleGroupLite) => {
  titleGroupStore.id = titleGroup.id
  titleGroupStore.content_type = titleGroup.content_type
  emit('done', titleGroup)
}
const createNew = () => {
  action.value = 'create'
}
</script>
<style scoped>
.name-input {
  width: 40%;
}
</style>
