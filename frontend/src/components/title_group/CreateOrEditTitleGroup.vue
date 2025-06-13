<template>
  <Form v-slot="$form" :initialValues="titleGroupForm" :resolver @submit="sendTitleGroup" validateOnSubmit :validateOnValueUpdate="false" validateOnBlur>
    <div class="line" v-if="content_type == 'software'">
      <FloatLabel>
        <InputNumber size="small" v-model="titleGroupForm.master_group_id" name="master_group_id" :format="false" />
        <label for="master_group_id">{{ t('master_group.master_group_id') }}</label>
      </FloatLabel>
    </div>
    <div class="line">
      <div class="name">
        <FloatLabel>
          <InputText size="small" v-model="titleGroupForm.name" name="name" />
          <label for="name">{{ t('general.name') }}</label>
        </FloatLabel>
        <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
          {{ $form.name.error?.message }}
        </Message>
      </div>
      <div>
        <FloatLabel>
          <Select
            v-model="titleGroupForm.category"
            inputId="category"
            :options="selectableCategories[content_type]"
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
      <div v-if="content_type == 'software'">
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
      <DatePicker v-model="original_release_date" showIcon :showOnFocus="false" inputId="original_release_date" size="small" name="original_release_date" />
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
    <div class="screenshots input-list" v-if="content_type == 'software'">
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
import { createTitleGroup, type ContentType, type TitleGroup, type TitleGroupCategory, type UserCreatedTitleGroup } from '@/services/api/torrentService'
import { createArtists, type Artist, type ArtistLite, type UserCreatedAffiliatedArtist, type UserCreatedArtist } from '@/services/api/artistService'
import { useI18n } from 'vue-i18n'
import { getLanguages, getPlatforms, getArtistRoles, isValidUrl } from '@/services/helpers'
import { watch } from 'vue'

interface Props {
  content_type: ContentType
  initialTitleGroupForm: UserCreatedTitleGroup | null
}
const { content_type, initialTitleGroupForm = null } = defineProps<Props>()

const sendingTitleGroup = ref(false)

const titleGroupForm = ref<UserCreatedTitleGroup>({
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
  content_type: 'movie',
})
const affiliated_artists_names = ref<[string]>([''])

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
  movie: ['FeatureFilm', 'ShortFilm'],
  software: ['Program', 'Game'],
  collection: ['Other'],
  tv_show: [],
}

const { t } = useI18n()

const emit = defineEmits<{
  done: [titleGroup: TitleGroup]
}>()

// type FormErrors = {
//   [key in keyof UserCreatedTitleGroup]: UserCreatedTitleGroup[key] extends Array<unknown>
//     ? { message: string }[][]
//     : { message: string }[]
// }
const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTitleGroup, { message: string }[]>> = {}

  if (values.name.length < 5) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }
  if (!titleGroupForm.value.category && selectableCategories[titleGroupForm.value.content_type]) {
    errors.category = [{ message: t('error.select_category') }]
  }
  //TODO config: the minimum amount of tags required should be taken from the global config file
  if (values.tags == '' || values.tags.split(',').length - 1 < 1) {
    errors.tags = [{ message: t('error.enter_at_least_x_tags', [2]) }]
  }
  if (values.description.length < 10) {
    errors.description = [{ message: t('error.write_more_than_x_chars', [10]) }]
  }
  if (values.platform == '') {
    errors.platform = [{ message: t('error.select_platform') }]
  }
  if (content_type !== 'music' && values.original_language == '') {
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
  titleGroupForm.value.content_type = content_type
  titleGroupForm.value.tags = tagsString.value.trim().split(',')
  titleGroupForm.value.screenshots = titleGroupForm.value.screenshots.filter((screenshot) => screenshot.trim() !== '')
  // create artists that need to be created
  const artistsToCreate: UserCreatedArtist[] = []
  titleGroupForm.value.affiliated_artists.forEach((artist, index) => {
    if (artist.artist_id === 0) {
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
  }
  titleGroupForm.value.affiliated_artists.forEach((artist) => {
    if (artist.artist_id === 0) {
      artist.artist_id = createdArtists[0].id
      createdArtists.shift()
    }
  })

  const formattedTitleGroupForm = JSON.parse(JSON.stringify(titleGroupForm.value))
  createTitleGroup(formattedTitleGroupForm)
    .then((data) => {
      emit('done', data)
    })
    .finally(() => {
      sendingTitleGroup.value = false
    })
}
// const onFormSubmit = ({ valid }: FormSubmitEvent) => {
//   if (valid) {
//    emit('validated', { ...titleGroupForm.value, content_type })
//   }
// }
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

watch(
  () => initialTitleGroupForm,
  (newValue) => {
    if (newValue !== null) {
      if (newValue.affiliated_artists.length === 0) {
        newValue.affiliated_artists = titleGroupForm.value.affiliated_artists
      }
      titleGroupForm.value = {
        ...titleGroupForm.value,
        ...newValue,
      }
      // TODO: make the form know that the inputs have been updated, it is a primevue form issue
      // https://github.com/primefaces/primevue/issues/7784
    }
  },
  { immediate: true },
)
</script>
<style scoped>
.description {
  width: 100%;
  height: 10em;
}

.name {
  width: 50%;

  input {
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
