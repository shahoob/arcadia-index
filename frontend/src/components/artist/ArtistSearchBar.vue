<template>
  <AutoComplete
    v-model="name"
    :suggestions="foundArtists"
    @complete="search"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="artistSelected"
  >
    <template #option="slotProps">
      <div>{{ slotProps.option.name }}</div>
    </template>
  </AutoComplete>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { AutoComplete, type AutoCompleteOptionSelectEvent } from 'primevue'
import { searchArtistsLite, type ArtistLite } from '@/services/api/artistService'

const props = defineProps<{
  placeholder: string
  clearInputOnSelect: boolean
}>()

const emit = defineEmits<{
  artistSelected: [ArtistLite]
}>()

const name = ref('')
const foundArtists = ref<ArtistLite[]>()

const artistSelected = (event: AutoCompleteOptionSelectEvent) => {
  emit('artistSelected', event.value)
  if (props.clearInputOnSelect) {
    name.value = ''
  }
}

const search = () => {
  if (name.value !== '') {
    console.log('Searching for:', name.value)
    searchArtistsLite(name.value).then((artists) => {
      foundArtists.value = artists
    })
  }
}
</script>
