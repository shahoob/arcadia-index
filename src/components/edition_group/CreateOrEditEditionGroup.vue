<template>
  <Form
    v-slot="$form"
    :initialValues="editionGroupForm"
    :resolver
    @submit="onFormSubmit"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
  >
    <div class="line">
      <FloatLabel>
        <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
          {{ $form.name.error?.message }}
        </Message>
        <InputText size="small" v-model="editionGroupForm.name" name="name" />
        <label for="name">Name</label>
      </FloatLabel>
      <FloatLabel>
        <Message v-if="$form.distributor?.invalid" severity="error" size="small" variant="simple">
          {{ $form.distributor.error?.message }}
        </Message>
        <InputText size="small" v-model="editionGroupForm.distributor" name="distributor" />
        <label for="distributor">Distributor</label>
      </FloatLabel>
      <FloatLabel>
        <Message v-if="$form.source?.invalid" severity="error" size="small" variant="simple">
          {{ $form.source.error?.message }}
        </Message>
        <Select
          v-model="editionGroupForm.source"
          inputId="source"
          :options="selectableSources[titleGroup.content_type]"
          class="select-source"
          size="small"
          name="source"
        />
        <label for="source">Source</label>
      </FloatLabel>
    </div>
    <FloatLabel>
      <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
        {{ $form.description.error?.message }}
      </Message>
      <Textarea
        v-model="editionGroupForm.description"
        name="description"
        class="description"
        autoResize
        rows="5"
      />
      <label for="description">Description</label>
    </FloatLabel>
    <div>
      <label for="release_date" class="block">Realease date</label>
      <Message v-if="$form.release_date?.invalid" severity="error" size="small" variant="simple">
        {{ $form.release_date.error?.message }}
      </Message>
      <DatePicker
        v-model="editionGroupForm.release_date"
        showIcon
        :showOnFocus="false"
        inputId="release_date"
        size="small"
        dateFormat="yy-mm-dd"
        name="release_date"
      />
    </div>
    <div class="covers input-list">
      <label>Covers</label>
      <div v-for="(link, index) in editionGroupForm.covers" :key="index">
        <Message
          v-if="$form.covers?.[index]?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $form.covers[index].error?.message }}
        </Message>
        <InputText
          size="small"
          v-model="editionGroupForm.covers[index]"
          :name="`covers[${index}]`"
        />
        <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
        <Button
          v-if="index != 0 || editionGroupForm.covers.length > 1"
          @click="removeCover(index)"
          icon="pi pi-minus"
          size="small"
        />
      </div>
    </div>
    <div class="external-links input-list">
      <label>External Links</label>
      <div v-for="(link, index) in editionGroupForm.external_links" :key="index">
        <Message
          v-if="$form.external_links?.[index]?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $form.external_links[index].error?.message }}
        </Message>
        <InputText
          size="small"
          v-model="editionGroupForm.external_links[index]"
          :name="`external_links[${index}]`"
        />
        <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
        <Button
          v-if="index != 0 || editionGroupForm.external_links.length > 1"
          @click="removeLink(index)"
          icon="pi pi-minus"
          size="small"
        />
      </div>
    </div>
    <div class="flex justify-content-center">
      <Button
        label="Validate edition"
        icon="pi pi-check"
        size="small"
        class="validate-button"
        type="submit"
        :loading="sendingEditionGroup"
      />
    </div>
  </Form>
</template>

<script lang="ts">
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import Message from 'primevue/message'
import { Form } from '@primevue/forms'
import { isValidUrl } from '@/services/helpers'

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
  },
  props: { titleGroup: {}, sendingEditionGroup: { default: false } },
  data() {
    return {
      editionGroupForm: {
        name: '',
        description: '',
        external_links: [''],
        covers: [''],
        release_date: null,
        title_group_id: 0,
        source: '',
        distributor: '',
        additional_information: {},
      },
      selectableSources: {
        Book: ['Web', 'CD', 'PhysicalBook'],
        Music: ['Web', 'CD', 'Vinyl'],
        Movie: ['Web', 'BluRay'],
        Software: ['Web'],
        'TV-Show': ['Web', 'BluRay', 'DVD9'],
        Collection: ['Web'],
      },
    }
  },
  methods: {
    resolver({ values }) {
      const errors = {}

      if (values.name.length < 5) {
        errors.name = [{ message: 'Write more than 5 characters' }]
      }
      if (values.distributor.length < 2) {
        errors.distributor = [{ message: 'Write more than 2 characters' }]
      }
      if (values.source == '') {
        errors.source = [{ message: 'Select a source' }]
      }
      if (!values.release_date) {
        errors.release_date = [{ message: 'Select a date' }]
      }
      if (values.description.length < 10) {
        errors.description = [{ message: 'Write more than 10 characters' }]
      }
      values.external_links.forEach((link, index) => {
        if (!isValidUrl(link)) {
          if (!('external_links' in errors)) {
            errors.external_links = []
          }
          errors.external_links[index] = [{ message: `Not a valid URL.` }]
        }
      })
      values.covers.forEach((link, index) => {
        if (!isValidUrl(link)) {
          if (!('covers' in errors)) {
            errors.covers = []
          }
          errors.covers[index] = [{ message: `Not a valid URL.` }]
        }
      })

      return {
        errors,
      }
    },
    onFormSubmit({ valid }) {
      if (valid) {
        this.$emit('validated', this.editionGroupForm)
      }
    },
    addCover() {
      this.editionGroupForm.covers.push('')
    },
    removeCover(index: Number) {
      this.editionGroupForm.covers.splice(index, 1)
    },
    addLink() {
      this.editionGroupForm.external_links.push('')
    },
    removeLink(index: Number) {
      this.editionGroupForm.external_links.splice(index, 1)
    },
  },
  created() {
    if (this.titleGroup.id) {
      this.editionGroupForm.title_group_id = this.titleGroup.id
    }
  },
}
</script>
<style scoped>
.description {
  width: 100%;
  height: 10em;
}
.select-source {
  width: 150px;
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
.p-floatlabel {
  margin-bottom: 30px;
}
.validate-button {
  margin-top: 20px;
}
</style>
