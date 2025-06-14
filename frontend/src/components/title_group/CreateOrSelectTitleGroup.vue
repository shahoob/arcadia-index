<template>
  <div class="title" v-if="action == 'select'">
    {{ t('title_group.select_title') }}
    <span class="alternative" @click="action = 'create'">({{ t('general.or_create_one') }})</span>
  </div>
  <div class="title" v-if="action == 'create'">
    {{ t('title_group.create_title') }}
    <span class="alternative" @click="action = 'select'">({{ t('general.or_select_one') }})</span>
  </div>

  <div id="select-title-group" v-if="action == 'select'">
    <FloatLabel>
      <InputNumber size="small" v-model="titleGroupId" name="id" :format="false" />
      <label for="id">{{ t('title_group.title_group_id') }}</label>
    </FloatLabel>
    <div class="flex justify-content-center">
      <Button
        v-if="action == 'select'"
        :label="t('general.submit')"
        @click="sendSelectedTitleGroup"
        icon="pi pi-check"
        size="small"
        class="validate-button"
        :loading="gettingTitleGroupInfo"
      />
    </div>
  </div>
  <div id="create-title-group" v-if="action == 'create'">
    <FloatLabel>
      <Select v-model="content_type" inputId="content_type" :options="selectableContentTypes" class="select" size="small">
        <template #option="slotProps">
          <span>{{ t(`title_group.content_type.${slotProps.option}`) }}</span>
        </template>
        <template #value="slotProps">
          <span v-if="slotProps.value">
            {{ t(`title_group.content_type.${slotProps.value}`) }}
          </span>
        </template>
      </Select>
      <label for="content_type">{{ t('title_group.content_type.content_type') }}</label>
    </FloatLabel>
    <div class="external-db-inputs-wrapper">
      <!-- <div class="external-db-inputs" v-if="content_type == 'movie'">
        <ExternalDBSearchBar inputPlaceholder="TMDB id" database="tmdb/movie" @dataFound="externalDBDataFound" />
        or
        <ExternalDBSearchBar inputPlaceholder="IMDB id" database="imdb/movie" @dataFound="externalDBDataFound" />
      </div> -->
      <!-- <div class="external-db-inputs" v-if="content_type == 'tv_show'">
        <ExternalDBSearchBar inputPlaceholder="TVDB id" database="tvdb" @dataFound="externalDBDataFound" />
        or
        <ExternalDBSearchBar inputPlaceholder="TMDB id" database="tmdb/tv" @dataFound="externalDBDataFound" />
        or
        <ExternalDBSearchBar inputPlaceholder="IMDB id" database="imdb/tv" @dataFound="externalDBDataFound" />
      </div> -->
      <!-- <div class="external-db-inputs" v-if="content_type == 'music'">
        <ExternalDBSearchBar inputPlaceholder="Musicbrainz id" database="musicbrainz" @dataFound="externalDBDataFound" />
        or
        <ExternalDBSearchBar inputPlaceholder="Discogs id" database="discogs" @dataFound="externalDBDataFound" />
      </div> -->
      <div class="external-db-inputs" v-if="content_type == 'book'">
        <ExternalDBSearchBar inputPlaceholder="Open Library id" database="openlibrary" @dataFound="externalDBDataFound" />
      </div>
    </div>
    <div>
      <CreateOrEditTitleGroup :content_type="content_type" @done="titleGroupCreated" :initialTitleGroupForm />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { InputNumber } from 'primevue'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { getTitleGroupLite, type ContentType, type TitleGroup, type UserCreatedEditionGroup, type UserCreatedTitleGroup } from '@/services/api/torrentService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import CreateOrEditTitleGroup from '../title_group/CreateOrEditTitleGroup.vue'
import { useI18n } from 'vue-i18n'
import ExternalDBSearchBar from './ExternalDBSearchBar.vue'
import type { ExternalDBData } from '@/services/api/externalDatabasesService'

const action = ref('select') // create | select
const titleGroupId = ref<number | null>(null)
const selectableContentTypes: ContentType[] = ['movie', 'tv_show', 'music', 'software', 'book', 'collection']
const content_type = ref<ContentType>('movie') // consider either
const gettingTitleGroupInfo = ref(false)
const initialTitleGroupForm = ref<UserCreatedTitleGroup | null>(null)
const titleGroupStore = useTitleGroupStore()

const { t } = useI18n()

const emit = defineEmits<{
  gotEditionData: [editionGroup: UserCreatedEditionGroup]
  done: [titleGroup?: TitleGroup]
}>()

const externalDBDataFound = (data: ExternalDBData) => {
  initialTitleGroupForm.value = data.title_group
  //TODO: emit if we also get some edition data
}

const sendSelectedTitleGroup = async (): Promise<void> => {
  gettingTitleGroupInfo.value = true
  if (!titleGroupStore.id && titleGroupId.value) {
    const titleGroupLite = await getTitleGroupLite(titleGroupId.value).finally(() => {
      gettingTitleGroupInfo.value = false
    })
    if (titleGroupLite) {
      titleGroupStore.id = titleGroupLite.id
      titleGroupStore.edition_groups = titleGroupLite.edition_groups
      titleGroupStore.content_type = titleGroupLite.content_type
    }
  }
  if (titleGroupStore.id) {
    emit('done')
  }
}

const titleGroupCreated = async (titleGroup: TitleGroup) => {
  titleGroupStore.id = titleGroup.id
  titleGroupStore.content_type = titleGroup.content_type
  emit('done', titleGroup)
}

onMounted(() => {
  if (titleGroupStore.id) {
    titleGroupId.value = titleGroupStore.id
  }
})
</script>
<style scoped>
.title {
  margin-bottom: 25px;
}

.title .alternative {
  font-size: 0.8em;
  color: var(--color-secondary);
  cursor: pointer;
}

.p-floatlabel {
  margin-top: 30px;
}

.select {
  width: 200px;
}

.external-db-inputs-wrapper {
  display: flex;
  align-items: center;
}

.external-db-inputs {
  display: flex;
  align-items: center;
  margin-left: -10px;
  margin-top: 30px;
}

.external-db-inputs .p-floatlabel {
  margin: 0px 10px;
}

.validate-button {
  margin-top: 20px;
}
</style>
