<template>
  <AutoComplete
    v-model="name"
    :suggestions="foundTitleGroups"
    @complete="search"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="titleGroupSelected"
    @input="onInput"
    fluid
  >
    <template #option="slotProps">
      <!-- <div>{{ slotProps.option.name }} ({{ slotProps.option.original_release_date.substring(0, 4) }})</div> -->
      <TitleGroupSlimHeader :title_group="slotProps.option" />
    </template>
  </AutoComplete>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import { AutoComplete, type AutoCompleteOptionSelectEvent } from 'primevue'
import { searchTitleGroupLite, type TitleGroupLite } from '@/services/api/torrentService'
import TitleGroupSlimHeader from './TitleGroupSlimHeader.vue'

const props = defineProps<{
  placeholder: string
  clearInputOnSelect: boolean
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [string]
  titleGroupSelected: [TitleGroupLite]
}>()

const name = ref('')

watch(
  () => props.modelValue,
  (newValue) => {
    name.value = newValue
  },
  { immediate: true },
)

const foundTitleGroups = ref<TitleGroupLite[]>()

const titleGroupSelected = (event: AutoCompleteOptionSelectEvent) => {
  const selectedTitleGroupName = (event.value as TitleGroupLite).name
  emit('titleGroupSelected', event.value)
  emit('update:modelValue', selectedTitleGroupName)
  if (props.clearInputOnSelect) {
    name.value = ''
  } else {
    name.value = selectedTitleGroupName
  }
}

const onInput = () => {
  emit('update:modelValue', name.value)
}

const search = () => {
  if (name.value !== '') {
    searchTitleGroupLite(name.value).then((titleGroups) => {
      foundTitleGroups.value = titleGroups
    })
  } else {
    foundTitleGroups.value = []
  }
}
</script>
<style scoped></style>
