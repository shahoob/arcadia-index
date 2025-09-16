<template>
  <div class="tag-input-container">
    <InputText v-model="newTag" placeholder="Add a new tag (press enter)" @keyup.enter="addTag" size="small" />
    <div v-for="(tag, index) in modelValue" :key="index" class="tag-chip">
      <Chip>
        {{ tag }}
        <i class="pi pi-times-circle cursor-pointer" style="font-size: 0.8rem" @click="removeTag(index)" />
      </Chip>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Chip, InputText } from 'primevue'
import { showToast } from '@/main'

const props = defineProps({
  modelValue: {
    type: Array<string>,
    default: () => [],
  },
})
const emit = defineEmits(['update:modelValue'])

const newTag = ref('')

const addTag = () => {
  if (props.modelValue.includes(newTag.value.trim())) {
    showToast('error', "You can't enter duplicate tags", 'error', 3000)
    return
  }
  if (newTag.value.trim() !== '') {
    const updatedTags = [...props.modelValue, newTag.value.trim()]
    emit('update:modelValue', updatedTags)
    newTag.value = ''
  }
}

const removeTag = (index: number) => {
  const updatedTags = props.modelValue.filter((_, i) => i !== index)
  emit('update:modelValue', updatedTags)
}
</script>

<style scoped>
.tag-input-container {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 20px;
}
</style>
