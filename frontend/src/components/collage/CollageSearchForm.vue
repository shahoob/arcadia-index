<template>
  <ContentContainer>
    <Form @submit="fetchCollages">
      <FloatLabel>
        <InputText v-model="form.name" name="name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('general.search')" type="submit" :loading />
      </div>
    </Form>
  </ContentContainer>
</template>
<script setup lang="ts">
import { searchCollages, type CollageSearchResponse, type SearchCollagesQuery } from '@/services/api/collageService'
import ContentContainer from '../ContentContainer.vue'
import { InputText, Button, FloatLabel } from 'primevue'
import { Form } from '@primevue/forms'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const form = ref<SearchCollagesQuery>({
  name: '',
  page: 1,
  page_size: 50,
  tags: [],
})
const loading = ref(false)
const collageSearchResponse = ref<CollageSearchResponse>()

onMounted(async () => {
  await fetchCollages()
})

const fetchCollages = async () => {
  loading.value = true
  collageSearchResponse.value = await searchCollages(form.value).finally(() => (loading.value = false))
  emit('gotResults', collageSearchResponse.value)
}

const emit = defineEmits<{
  gotResults: [CollageSearchResponse]
}>()

// defineProps<{
//   collage: Collage
// }>()
</script>
<style scoped>
#collage-sidebar {
  display: block;
  > * {
    margin-bottom: 10px;
  }
}
</style>
