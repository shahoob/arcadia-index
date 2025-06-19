<template>
  <Form
    class="form"
    v-slot="$form"
    :initialValues="titleGroupForm"
    :resolver
    @submit="sendTitleGroup"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
    ref="formRef"
  >
    <FloatLabel style="margin-top: 0px">
      <Select
        v-model="titleGroupForm.content_type"
        @update:model-value="(content_type) => (titleGroupStore.content_type = content_type)"
        inputId="content_type"
        :options="selectableContentTypes"
        class="select"
        size="small"
      >
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
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'music'">
        <ExternalDBSearchBar inputPlaceholder="Musicbrainz url" database="musicbrainz" @dataFound="externalDBDataFound" />
        <!-- or -->
        <!-- <ExternalDBSearchBar inputPlaceholder="Discogs id" database="discogs" @dataFound="externalDBDataFound" /> -->
      </div>
      <!-- <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'book'">
        <ExternalDBSearchBar inputPlaceholder="Open Library id" database="openlibrary" />
      </div> -->
    </div>
    <div class="name">
      <FloatLabel>
        <InputText size="small" v-model="titleGroupForm.name" name="name" class="name-input" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
        {{ $form.name.error?.message }}
      </Message>
    </div>
    <div v-if="titleGroupForm.content_type !== null">
      <div class="line" v-if="titleGroupForm.content_type == 'software'">
        <FloatLabel>
          <InputNumber size="small" v-model="titleGroupForm.master_group_id" name="master_group_id" :format="false" />
          <label for="master_group_id">{{ t('master_group.master_group_id') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <div v-if="selectableCategories[titleGroupForm.content_type].length !== 0">
          <FloatLabel>
            <Select
              v-model="titleGroupForm.category"
              inputId="category"
              :options="selectableCategories[titleGroupForm.content_type]"
              size="small"
              name="category"
              class="select"
            />
            <label for="category">{{ t('general.category') }}</label>
          </FloatLabel>
          <Message v-if="$form.category?.invalid" severity="error" size="small" variant="simple">
            {{ $form.category.error?.message }}
          </Message>
        </div>
        <div class="tags">
          <FloatLabel>
            <InputText size="small" v-model="tagsString" name="tags" />
            <label for="tags">{{ t('general.tags_comma_separated') }}</label>
          </FloatLabel>
          <Message v-if="$form.tags?.invalid" severity="error" size="small" variant="simple">
            {{ $form.tags.error?.message }}
          </Message>
        </div>
      </div>
      <div>
        <FloatLabel>
          <Textarea v-model="titleGroupForm.description" name="description" class="description" autoResize rows="5" />
          <label for="description">{{ t('general.description') }}</label>
        </FloatLabel>
        <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
          {{ $form.description.error?.message }}
        </Message>
      </div>
      <div class="line">
        <div v-if="titleGroupForm.content_type == 'software'">
          <FloatLabel>
            <Select v-model="titleGroupForm.platform" inputId="platform" :options="getPlatforms()" class="select" size="small" name="platform" filter />
            <label for="platform">{{ t('title_group.platform') }}</label>
          </FloatLabel>
          <Message v-if="$form.platform?.invalid" severity="error" size="small" variant="simple">
            {{ $form.platform.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel>
            <Select
              v-model="titleGroupForm.original_language"
              inputId="original_language"
              :options="getLanguages()"
              class="select"
              size="small"
              name="original_language"
              filter
            />
            <label for="original_language">{{ t('general.original_language') }}</label>
          </FloatLabel>
          <Message v-if="$form.original_language?.invalid" severity="error" size="small" variant="simple">
            {{ $form.original_language.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel>
            <Select
              v-model="titleGroupForm.country_from"
              inputId="country_from"
              :options="selectableCountries"
              class="select"
              size="small"
              name="country_from"
              filter
            />
            <label for="country_from">{{ t('general.country') }}</label>
          </FloatLabel>
          <Message v-if="$form.country_from?.invalid" severity="error" size="small" variant="simple">
            {{ $form.country_from.error?.message }}
          </Message>
        </div>
      </div>
      <div class="original-release-date">
        <label for="original_release_date" class="block">{{ t('title_group.original_release_date') }}</label>
        <DatePicker
          v-model="original_release_date"
          showIcon
          iconDisplay="input"
          inputId="original_release_date"
          size="small"
          dateFormat="yy-mm-dd"
          name="original_release_date"
        />
        <Message v-if="$form.original_release_date?.invalid" severity="error" size="small" variant="simple">
          {{ $form.original_release_date.error?.message }}
        </Message>
      </div>
      <div class="artists input-list">
        <label>{{ t('artist.artist', 2) }}</label>
        <div v-for="(_link, index) in affiliated_artists_names" :key="index">
          <ArtistSearchBar
            :placeholder="t('artist.name')"
            :clearInputOnSelect="false"
            v-model="affiliated_artists_names[index]"
            @artistSelected="(event) => artistSelected(event, index)"
          />
          <InputText
            size="small"
            v-model="titleGroupForm.affiliated_artists[index].nickname"
            :placeholder="t('artist.nickname')"
            class="artist"
            v-tooltip.top="t('artist.nickname_explanation')"
          />
          <MultiSelect
            v-model="titleGroupForm.affiliated_artists[index].roles"
            :options="getArtistRoles()"
            size="small"
            class="select"
            :placeholder="t('artist.role.role', 2)"
          />
          <span v-if="titleGroupForm.affiliated_artists[index].artist_id !== 0" class="artist-creation-hint existing">
            {{ t('artist.existing_artist') }}
          </span>
          <span v-else-if="affiliated_artists_names[index] !== ''" class="artist-creation-hint new">{{ t('artist.new_artist') }}</span>
          <Button v-if="index == 0" @click="addAffiliatedArtist" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || affiliated_artists_names.length > 1" @click="removeAffiliatedArtist(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.affiliated_artists as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.affiliated_artists as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
      <div class="covers input-list">
        <label>{{ t('general.cover', 2) }}</label>
        <div v-for="(_link, index) in titleGroupForm.covers" :key="index">
          <InputText size="small" v-model="titleGroupForm.covers[index]" />
          <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.covers.length > 1" @click="removeCover(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.covers as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.covers as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
      <div class="screenshots input-list" v-if="titleGroupForm.content_type == 'software'">
        <label>{{ t('general.screenshots') }}</label>
        <div v-for="(_link, index) in titleGroupForm.screenshots" :key="index">
          <InputText size="small" v-model="titleGroupForm.screenshots[index]" />
          <Button v-if="index == 0" @click="addScreenshot" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.screenshots.length > 1" @click="removeScreenshot(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.screenshots as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.screenshots as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
      <div class="external-links input-list">
        <label>{{ t('general.external_link', 2) }}</label>
        <div v-for="(_link, index) in titleGroupForm.external_links" :key="index">
          <InputText size="small" v-model="titleGroupForm.external_links[index]" :name="`external_links[${index}]`" />
          <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.external_links.length > 1" @click="removeLink(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.external_links as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.external_links as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
    </div>
    <div class="flex justify-content-center">
      <Button label="Validate title" icon="pi pi-check" type="submit" size="small" class="validate-button" :loading="sendingTitleGroup" />
    </div>
  </Form>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
import { Form, type FormFieldState, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import MultiSelect from 'primevue/multiselect'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import Message from 'primevue/message'
import ArtistSearchBar from '../artist/ArtistSearchBar.vue'
import { InputNumber } from 'primevue'
import {
  createTitleGroup,
  type ContentType,
  type TitleGroup,
  type TitleGroupCategory,
  type TitleGroupLite,
  type UserCreatedEditionGroup,
  type UserCreatedTitleGroup,
} from '@/services/api/torrentService'
import { createArtists, type Artist, type ArtistLite, type UserCreatedAffiliatedArtist, type UserCreatedArtist } from '@/services/api/artistService'
import { useI18n } from 'vue-i18n'
import { getLanguages, getPlatforms, getArtistRoles, isValidUrl } from '@/services/helpers'
import { watch } from 'vue'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { onMounted } from 'vue'
import type { VNodeRef } from 'vue'
import ExternalDBSearchBar from './ExternalDBSearchBar.vue'
import type { ExternalDBData } from '@/services/api/externalDatabasesService'

interface Props {
  initialTitleGroupForm: UserCreatedTitleGroup | null
  initialTitleGroupName: string
}
const { initialTitleGroupForm = null, initialTitleGroupName = '' } = defineProps<Props>()
const titleGroupStore = ref(useTitleGroupStore())

const sendingTitleGroup = ref(false)

const selectableContentTypes: ContentType[] = ['movie', 'video', 'tv_show', 'music', 'podcast', 'software', 'book', 'collection']
type UserCreatedTitleGroupForm = Omit<UserCreatedTitleGroup, 'content_type'> & {
  content_type: ContentType | null
}
const titleGroupForm = ref<UserCreatedTitleGroupForm>({
  name: '',
  description: '',
  original_language: '',
  original_release_date: '',
  covers: [''],
  screenshots: [''],
  external_links: [''],
  category: null,
  country_from: '',
  name_aliases: [],
  affiliated_artists: [{ artist_id: 0, nickname: null, roles: [], title_group_id: 0 }],
  tags: [],
  master_group_id: null,
  platform: null,
  embedded_links: {},
  content_type: null,
})
const affiliated_artists_names = ref<[string]>([''])
const formRef = ref<VNodeRef | null>(null)

const original_release_date = computed({
  get() {
    const isValidDateStr = !isNaN(Date.parse(titleGroupForm.value.original_release_date ?? ''))
    return isValidDateStr ? new Date(titleGroupForm.value.original_release_date ?? '') : null
  },
  set(newValue) {
    titleGroupForm.value.original_release_date = newValue?.toISOString() ?? ''
  },
})

const tagsString = ref('')
const selectableCountries = ['France', 'UK', 'USA', 'Scotland']
const selectableCategories: Record<ContentType, TitleGroupCategory[]> = {
  book: ['Illustrated', 'Periodical', 'Book', 'Article', 'Manual'],
  music: ['Single', 'Album', 'Ep'],
  podcast: [],
  movie: ['FeatureFilm', 'ShortFilm'],
  video: [],
  software: ['Program', 'Game'],
  collection: ['Other'],
  tv_show: [],
}

const { t } = useI18n()

const emit = defineEmits<{
  done: [titleGroup: TitleGroup | TitleGroupLite]
  editionGroupDataFound: [editionGroup: UserCreatedEditionGroup]
}>()

// type FormErrors = {
//   [key in keyof UserCreatedTitleGroup]: UserCreatedTitleGroup[key] extends Array<unknown>
//     ? { message: string }[][]
//     : { message: string }[]
// }
const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTitleGroup, { message: string }[]>> = {}

  if (!titleGroupForm.value.content_type) {
    errors.content_type = [{ message: t('error.select_content_type') }]
    return { errors }
  }
  if (values.name.length < 1) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [0]) }]
  }
  if (!titleGroupForm.value.category && selectableCategories[titleGroupForm.value.content_type]) {
    errors.category = [{ message: t('error.select_category') }]
  }
  //TODO config: the minimum amount of tags required should be taken from the global config file
  if (tagsString.value == '' || tagsString.value.split(',').length - 1 < 1) {
    errors.tags = [{ message: t('error.enter_at_least_x_tags', [2]) }]
  }
  if (values.description.length < 10) {
    errors.description = [{ message: t('error.write_more_than_x_chars', [10]) }]
  }
  if (values.platform == '') {
    errors.platform = [{ message: t('error.select_platform') }]
  }
  if (titleGroupForm.value.content_type !== 'music' && values.original_language == '') {
    errors.original_language = [{ message: t('error.select_language') }]
  }
  if (values.country_from == '') {
    errors.country_from = [{ message: t('error.select_country') }]
  }
  if (values.original_release_date == '') {
    errors.original_release_date = [{ message: t('error.select_date') }]
  }
  affiliated_artists_names.value.forEach((artist_name: string, index: number) => {
    if (artist_name === '') {
      if (!('affiliated_artists' in errors)) {
        errors.affiliated_artists = []
      }
      errors.affiliated_artists![index] = { message: t('error.invalid_name') }
    }
  })
  titleGroupForm.value.affiliated_artists.forEach((artist: UserCreatedAffiliatedArtist, index: number) => {
    if (artist.roles.length === 0) {
      if (!('affiliated_artists' in errors)) {
        errors.affiliated_artists = []
      }
      errors.affiliated_artists![index] = {
        message: t('error.artist_must_have_at_lease_one_role'),
      }
    }
  })
  values.external_links.forEach((link: string, index: number) => {
    if (!isValidUrl(link)) {
      if (!('external_links' in errors)) {
        errors.external_links = []
      }
      errors.external_links![index] = { message: t('error.invalid_url') }
    }
  })
  //TODO: should be values.covers, but somehow it is undefined
  titleGroupForm.value.covers.forEach((link: string, index: number) => {
    if (!isValidUrl(link)) {
      if (!('covers' in errors)) {
        errors.covers = []
      }
      errors.covers![index] = { message: t('error.invalid_url') }
    }
  })
  if (values.screenshots) {
    values.screenshots.forEach((link: string, index: number) => {
      if (!isValidUrl(link)) {
        if (!('screenshots' in errors)) {
          errors.screenshots = []
        }
        errors.screenshots![index] = { message: t('error.invalid_url') }
      }
    })
  }
  return {
    errors,
  }
}
const artistSelected = (artist: ArtistLite, index: number) => {
  titleGroupForm.value.affiliated_artists[index].artist_id = artist.id
}

const sendTitleGroup = async ({ valid }: FormSubmitEvent) => {
  if (!valid) {
    return
  }
  sendingTitleGroup.value = true
  titleGroupForm.value.tags = tagsString.value.trim().split(',')
  titleGroupForm.value.screenshots = titleGroupForm.value.screenshots.filter((screenshot) => screenshot.trim() !== '')
  // create artists that need to be created
  const artistsToCreate: UserCreatedArtist[] = []
  titleGroupForm.value.affiliated_artists.forEach((artist, index) => {
    if (artist.artist_id === 0 && affiliated_artists_names.value[index] !== '') {
      artistsToCreate.push({
        name: affiliated_artists_names.value[index],
        pictures: [],
        description: '',
      })
    }
  })
  let createdArtists: Artist[] = []
  if (artistsToCreate.length !== 0) {
    createdArtists = await createArtists(artistsToCreate)
    titleGroupForm.value.affiliated_artists.forEach((artist) => {
      if (artist.artist_id === 0) {
        artist.artist_id = createdArtists[0].id
        createdArtists.shift()
      }
    })
  }
  // removing the artists that haven't been created (empty inputs)
  titleGroupForm.value.affiliated_artists = titleGroupForm.value.affiliated_artists.filter((aa) => aa.artist_id !== 0)
  const formattedTitleGroupForm = JSON.parse(JSON.stringify(titleGroupForm.value))
  createTitleGroup(formattedTitleGroupForm)
    .then((data) => {
      emit('done', data)
    })
    .finally(() => {
      sendingTitleGroup.value = false
    })
}
const addAffiliatedArtist = () => {
  affiliated_artists_names.value.push('')
  titleGroupForm.value.affiliated_artists.push({
    artist_id: 0,
    nickname: null,
    roles: [],
    title_group_id: 0,
  })
}
const removeAffiliatedArtist = (index: number) => {
  affiliated_artists_names.value.splice(index, 1)
  titleGroupForm.value.affiliated_artists.splice(index, 1)
}
const addLink = () => {
  titleGroupForm.value.external_links.push('')
}
const removeLink = (index: number) => {
  titleGroupForm.value.external_links.splice(index, 1)
}
const addCover = () => {
  titleGroupForm.value.covers.push('')
}
const removeCover = (index: number) => {
  titleGroupForm.value.covers.splice(index, 1)
}
const addScreenshot = () => {
  titleGroupForm.value.screenshots.push('')
}
const removeScreenshot = (index: number) => {
  titleGroupForm.value.screenshots.splice(index, 1)
}

const externalDBDataFound = (data: ExternalDBData) => {
  if (data.title_group) {
    updateTitleGroupForm(data.title_group)
  }
  if (data.edition_group) {
    emit('editionGroupDataFound', data.edition_group)
  }
}
const updateTitleGroupForm = (form: UserCreatedTitleGroup) => {
  if (form.affiliated_artists.length === 0) {
    form.affiliated_artists = titleGroupForm.value.affiliated_artists
  }
  titleGroupForm.value = {
    ...titleGroupForm.value,
    ...form,
  }
  if (titleGroupForm.value.tags.length > 0) {
    tagsString.value = titleGroupForm.value.tags.join(',')
  }
  try {
    // some field is apparently undefined, the whole form seems to still get populated though
    formRef.value?.setValues(titleGroupForm.value)
  } catch {}
}

watch(
  () => initialTitleGroupForm,
  (newValue) => {
    if (newValue !== null) {
      updateTitleGroupForm(newValue)
    }
  },
  { immediate: true },
)
onMounted(() => {
  titleGroupForm.value.name = initialTitleGroupName
  formRef.value?.setFieldValue('name', initialTitleGroupName)
})
</script>
<style scoped>
.description {
  width: 100%;
  height: 10em;
}
.external-db-inputs-wrapper {
  display: flex;
  align-items: center;
}
.name {
  width: 50%;
  .name-input {
    width: 100%;
  }
}
.tags {
  width: 50%;
  input {
    width: 100%;
  }
}
.select {
  width: 200px;
}
.p-floatlabel {
  margin-top: 30px;
}
.original-release-date {
  margin-top: 30px;
}
.input-list {
  margin-top: 15px;
}
.input-list .p-component {
  margin-right: 5px;
  margin-bottom: 5px;
}
.input-list input {
  &:not(.artist) {
    width: 400px;
  }
  &.artist {
    width: 230px;
  }
}
.artist-creation-hint {
  margin-right: 5px;
  &.new {
    color: green;
  }
}
.validate-button {
  margin-top: 20px;
}
</style>
