<template>
  <Form
    v-slot="$form"
    :initialValues="titleGroupForm"
    :resolver
    @submit="onFormSubmit"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
  >
    <div class="line" v-if="content_type == 'Software'">
      <FloatLabel>
        <InputNumber
          size="small"
          v-model="titleGroupForm.master_group_id"
          name="master_group_id"
          :format="false"
        />
        <label for="master_group_id">Master group id</label>
      </FloatLabel>
    </div>
    <div class="line">
      <div class="name">
        <FloatLabel>
          <InputText size="small" v-model="titleGroupForm.name" name="name" />
          <label for="name">Name</label>
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
          <label for="category">Category</label>
        </FloatLabel>
        <Message v-if="$form.category?.invalid" severity="error" size="small" variant="simple">
          {{ $form.category.error?.message }}
        </Message>
      </div>
      <div class="tags">
        <FloatLabel>
          <InputText size="small" v-model="titleGroupForm.tags" name="tags" />
          <label for="tags">Tags (comma separated)</label>
        </FloatLabel>
        <Message v-if="$form.tags?.invalid" severity="error" size="small" variant="simple">
          {{ $form.tags.error?.message }}
        </Message>
      </div>
    </div>
    <div>
      <FloatLabel>
        <Textarea
          v-model="titleGroupForm.description"
          name="description"
          class="description"
          autoResize
          rows="5"
        />
        <label for="description">Description</label>
      </FloatLabel>
      <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
        {{ $form.description.error?.message }}
      </Message>
    </div>
    <div class="line">
      <div v-if="content_type == 'Software'">
        <FloatLabel>
          <Select
            v-model="titleGroupForm.platform"
            inputId="platform"
            :options="$getPlatforms()"
            class="select"
            size="small"
            name="platform"
            filter
          />
          <label for="platform">Platform</label>
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
            :options="$getLanguages()"
            class="select"
            size="small"
            name="original_language"
            filter
          />
          <label for="original_language">Original language</label>
        </FloatLabel>
        <Message
          v-if="$form.original_language?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
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
          <label for="country_from">Country from</label>
        </FloatLabel>
        <Message v-if="$form.country_from?.invalid" severity="error" size="small" variant="simple">
          {{ $form.country_from.error?.message }}
        </Message>
      </div>
    </div>
    <div class="original-release-date">
      <label for="original_release_date" class="block">Original release date</label>
      <DatePicker
        v-model="titleGroupForm.original_release_date"
        showIcon
        :showOnFocus="false"
        inputId="original_release_date"
        size="small"
        name="original_release_date"
      />
      <Message
        v-if="$form.original_release_date?.invalid"
        severity="error"
        size="small"
        variant="simple"
      >
        {{ $form.original_release_date.error?.message }}
      </Message>
    </div>
    <div class="covers input-list">
      <label>Covers</label>
      <div v-for="(link, index) in titleGroupForm.covers" :key="index">
        <InputText size="small" v-model="titleGroupForm.covers[index]" :name="`covers[${index}]`" />
        <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
        <Button
          v-if="index != 0 || titleGroupForm.covers.length > 1"
          @click="removeCover(index)"
          icon="pi pi-minus"
          size="small"
        />
        <Message
          v-if="$form.covers?.[index]?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $form.covers[index].error?.message }}
        </Message>
      </div>
    </div>
    <div class="screenshots input-list" v-if="content_type == 'Software'">
      <label>Screenshots</label>
      <div v-for="(link, index) in titleGroupForm.screenshots" :key="index">
        <InputText
          size="small"
          v-model="titleGroupForm.screenshots[index]"
          :name="`screenshots[${index}]`"
        />
        <Button v-if="index == 0" @click="addScreenshot" icon="pi pi-plus" size="small" />
        <Button
          v-if="index != 0 || titleGroupForm.screenshots.length > 1"
          @click="removeScreenshot(index)"
          icon="pi pi-minus"
          size="small"
        />
        <Message
          v-if="$form.screenshots?.[index]?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $form.screenshots[index].error?.message }}
        </Message>
      </div>
    </div>
    <div class="external-links input-list">
      <label>External Links</label>
      <div v-for="(link, index) in titleGroupForm.external_links" :key="index">
        <InputText
          size="small"
          v-model="titleGroupForm.external_links[index]"
          :name="`external_links[${index}]`"
        />
        <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
        <Button
          v-if="index != 0 || titleGroupForm.external_links.length > 1"
          @click="removeLink(index)"
          icon="pi pi-minus"
          size="small"
        />
        <Message
          v-if="$form.external_links?.[index]?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $form.external_links[index].error?.message }}
        </Message>
      </div>
    </div>
    <div class="flex justify-content-center">
      <Button
        label="Validate title"
        icon="pi pi-check"
        type="submit"
        size="small"
        class="validate-button"
        :loading="sendingTitleGroup"
      />
    </div>
  </Form>
</template>
<script lang="ts">
import { Form } from '@primevue/forms'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import Message from 'primevue/message'
import { InputNumber } from 'primevue'

export default {
  components: {
    Form,
    DatePicker,
    Button,
    FloatLabel,
    InputText,
    Textarea,
    Select,
    Message,
    InputNumber,
  },
  props: {
    content_type: {},
    initialTitleGroupForm: { default: {} },
    sendingTitleGroup: { default: false },
  },
  data() {
    return {
      titleGroupForm: {
        name: '',
        description: '',
        original_language: '',
        original_release_date: null,
        covers: [''],
        screenshots: [''],
        external_links: [''],
        category: '',
        country_from: '',
        name_aliases: [],
        affiliated_artists: [],
        tags: '',
        master_group_id: null,
        platform: null,
      },
      selectableCountries: ['France', 'UK', 'USA', 'Scotland'],
      selectableCategories: {
        Book: ['Illustrated', 'Periodical', 'Book', 'Article', 'Manual'],
        Music: ['Single', 'Album', 'Ep'],
        Movie: ['FeatureFilm', 'ShortFilm'],
        Software: ['Program', 'Game'],
        Collection: ['Other'],
      },
    }
  },

  methods: {
    resolver({ values }) {
      const errors = {}

      if (values.name.length < 5) {
        errors.name = [{ message: 'Write more than 5 characters' }]
      }
      if (values.category == '') {
        errors.category = [{ message: 'Select a category' }]
      }
      //TODO config: the minimum amount of tags required should be taken from the global config file
      if (values.tags == '' || values.tags.split(',').length - 1 < 1) {
        errors.tags = [{ message: 'Enter at least 2 tags' }]
      }
      if (values.description.length < 10) {
        errors.description = [{ message: 'Write more than 10 characters' }]
      }
      if (values.platform == '') {
        errors.platform = [{ message: 'Select a platform' }]
      }
      if (values.original_language == '') {
        errors.original_language = [{ message: 'Select a language' }]
      }
      if (values.country_from == '') {
        errors.country_from = [{ message: 'Select a country' }]
      }
      if (!values.original_release_date || values.original_release_date == '') {
        errors.original_release_date = [{ message: 'Select a date' }]
      }
      values.external_links.forEach((link, index) => {
        if (!this.$isValidUrl(link)) {
          if (!('external_links' in errors)) {
            errors.external_links = []
          }
          errors.external_links[index] = [{ message: `Not a valid URL.` }]
        }
      })
      values.covers.forEach((link, index) => {
        if (!this.$isValidUrl(link)) {
          if (!('covers' in errors)) {
            errors.covers = []
          }
          errors.covers[index] = [{ message: `Not a valid URL.` }]
        }
      })
      values.screenshots.forEach((link, index) => {
        if (!this.$isValidUrl(link)) {
          if (!('screenshots' in errors)) {
            errors.screenshots = []
          }
          errors.screenshots[index] = [{ message: `Not a valid URL.` }]
        }
      })
      console.log(errors)
      return {
        errors,
      }
    },
    onFormSubmit({ valid }) {
      if (valid) {
        this.$emit('validated', this.titleGroupForm)
      }
    },
    addLink() {
      this.titleGroupForm.external_links.push('')
    },
    removeLink(index: number) {
      this.titleGroupForm.external_links.splice(index, 1)
    },
    addCover() {
      this.titleGroupForm.covers.push('')
    },
    removeCover(index: number) {
      this.titleGroupForm.covers.splice(index, 1)
    },
    addScreenshot() {
      this.titleGroupForm.screenshots.push('')
    },
    removeScreenshot(index: number) {
      this.titleGroupForm.screenshots.splice(index, 1)
    },
  },
  created() {
    if (Object.keys(this.initialTitleGroupForm).length > 0) {
      this.titleGroupForm = this.initialTitleGroupForm
    }
  },
}
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
  width: 400px;
}
.validate-button {
  margin-top: 20px;
}
</style>
