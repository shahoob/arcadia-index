<template>
  <ContentContainer>
    <div id="torrent-search-inputs">
      <FloatLabel>
        <InputText
          class="title-group-name"
          size="small"
          v-model="searchForm.title_group.name"
          name="title_group_name"
        />
        <label for="title_group_name">{{ t('general.search_terms') }}</label>
      </FloatLabel>
      <!-- <FloatLabel>
        <InputText class="tags" size="small" v-model="searchForm.tags" name="tags" />
        <label for="tags">{{ t('general.tags_comma_separated') }}</label>
      </FloatLabel> -->
      <div class="staff-options" v-if="showStaffOptions">
        <div class="staff-option">
          <label>{{ t('torrent.staff_checked') }}</label>
          <Dropdown
            v-model="searchForm.torrent.staff_checked"
            :options="staffOptionChoices"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('general.both')"
            size="small"
          />
        </div>
        <div class="staff-option">
          <label>{{ t('general.reported') }}</label>
          <Dropdown
            v-model="searchForm.torrent.reported"
            :options="staffOptionChoices"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('general.both')"
            size="small"
          />
        </div>
      </div>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading :label="t('general.search')" @click="emit('search', searchForm)" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Button from 'primevue/button'
import { Dropdown } from 'primevue'
import type { TorrentSearch } from '@/services/api/torrentService'
import { onMounted } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  loading: boolean
  initialForm: TorrentSearch
  showStaffOptions: boolean
}>()

const emit = defineEmits<{
  search: [form: TorrentSearch]
}>()

const searchForm = ref<TorrentSearch>({
  title_group: { name: '' },
  torrent: { staff_checked: null },
  // tags: '',
})
const staffOptionChoices = ref([
  { label: t('general.yes'), value: true },
  { label: t('general.no'), value: false },
  { label: t('general.both'), value: null },
])

onMounted(async () => {
  searchForm.value = props.initialForm
})
</script>

<style>
#torrent-search-inputs {
  padding-top: 20px;
}
.title-group-name {
  width: 40%;
}
.tags {
  width: 30%;
}
.p-floatlabel {
  margin-bottom: 25px;
}
.staff-options {
  display: flex;
  .staff-option {
    display: flex;
    align-items: center;
    margin-right: 10px;
  }
  label {
    margin-right: 5px;
  }
}
</style>
