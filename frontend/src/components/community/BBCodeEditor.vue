<template>
  <FloatLabel style="width: 100%" variant="in">
    <Textarea
      v-model="content"
      rows="5"
      style="width: 100%"
      autoResize
      @value-change="$emit('value-change', content)"
      name="content"
    />
    <label for="in_label">{{ label }}</label>
  </FloatLabel>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { FloatLabel, Textarea } from 'primevue'

export default defineComponent({
  components: { FloatLabel, Textarea },
  props: { label: {} },
  data() {
    return {
      content: '',
    }
  },
})
</script>
