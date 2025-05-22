<template>
  <FloatLabel style="width: 100%" variant="in">
    <Textarea
      v-model="content"
      rows="5"
      style="width: 100%"
      autoResize
      @value-change="emit('valueChange', content)"
      name="content"
    />
    <label for="in_label">{{ label }}</label>
  </FloatLabel>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FloatLabel, Textarea } from 'primevue'
import { watch } from 'vue'

const props = defineProps<{
  label: string
  emptyInput: boolean
}>()

const emit = defineEmits<{
  inputEmptied: [boolean]
  valueChange: [string]
}>()

const content = ref('')

watch(
  () => props.emptyInput,
  (newVal) => {
    if (newVal) {
      content.value = ''
      emit('inputEmptied', true)
    }
  },
)
</script>
