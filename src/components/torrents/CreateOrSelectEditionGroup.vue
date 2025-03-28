<template>
  <div class="title" v-if="action == 'select'">
    Select edition group <span class="alternative" @click="action = 'create'">(or create one)</span>
  </div>
  <div class="title" v-if="action == 'create'">
    Create a new edition group
    <span class="alternative" @click="action = 'select'">(or select an existing one)</span>
  </div>
  <div id="select-title-group" v-if="action == 'select'">edition selector</div>
  <div v-if="action == 'create'">
    <div v-if="step > 0">
      <FloatLabel>
        <InputText v-model="editionGroupForm.name" name="name" />
        <label for="name">Name</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="editionGroupForm.distributor" name="distributor" />
        <label for="distributor">Distributor</label>
      </FloatLabel>
      <FloatLabel>
        <Textarea
          v-model="editionGroupForm.description"
          name="description"
          class="description"
          autoResize
          rows="5"
        />
        <label for="description">Description</label>
      </FloatLabel>
      <FloatLabel>
        <Select
          v-model="editionGroupForm.language"
          inputId="language"
          :options="selectableLanguages"
          class="select"
          filter
        />
        <label for="language">Language</label>
      </FloatLabel>
      <FloatLabel>
        <Select
          v-model="editionGroupForm.country_from"
          inputId="country_from"
          :options="selectableCountries"
          class="select"
          filter
        />
        <label for="country_from">Country from</label>
      </FloatLabel>
      <div>
        <label for="release_date" class="block">Realease date</label>
        <DatePicker
          v-model="editionGroupForm.release_date"
          showIcon
          :showOnFocus="false"
          inputId="release_date"
          dateFormat="yy-mm-dd"
        />
      </div>
      <div class="covers input-list">
        <label>Covers</label>
        <div v-for="(link, index) in editionGroupForm.covers" :key="index">
          <InputText v-model="editionGroupForm.covers[index]" />
          <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
          <Button
            v-if="editionGroupForm.covers.length != 0"
            @click="removeCover(index)"
            icon="pi pi-minus"
            size="small"
          />
        </div>
      </div>
      <div class="external-links input-list">
        <label>External Links</label>
        <div v-for="(link, index) in editionGroupForm.external_links" :key="index">
          <InputText v-model="editionGroupForm.external_links[index]" />
          <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
          <Button
            v-if="editionGroupForm.external_links.length != 0"
            @click="removeLink(index)"
            icon="pi pi-minus"
            size="small"
          />
        </div>
      </div>
    </div>
  </div>
  <div class="flex justify-content-center">
    <Button
      v-if="step == 3 || action == 'select'"
      label="Validate edition group"
      @click="validateEditionGroup"
      icon="pi pi-check"
      size="small"
      class="validate-button"
      :loading="creatingEditionGroup"
    />
  </div>
</template>

<script lang="ts">
import { InputNumber } from 'primevue'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { getExternalDatabaseData } from '@/services/api/externalDatabasesService'
import InputIcon from 'primevue/inputicon'
import IconField from 'primevue/iconfield'
import DatePicker from 'primevue/datepicker'
import { createTitleGroup } from '@/services/api/torrentService'

export default {
  components: {
    DatePicker,
    // eslint-disable-next-line vue/no-reserved-component-names
    Button,
    InputNumber,
    FloatLabel,
    InputText,
    // eslint-disable-next-line vue/no-reserved-component-names
    Textarea,
    // eslint-disable-next-line vue/no-reserved-component-names
    Select,
    InputIcon,
    IconField,
  },
  props: {},
  data() {
    return {
      action: 'select', // create | select
      titleGroupId: '',
      step: 1,
      manualCreation: false,
      editionGroupForm: {
        name: '',
        description: '',
        external_links: [''],
        covers: [''],
        release_date: null,
        country_from: '',
        title_group_id: 0,
        source: '',
        distributor: '',
        additional_information: {},
      },
      editionGroupId: '',
      selectableLanguages: ['English', 'French'],
      selectableCountries: ['USA', 'France', 'UK', 'Germany'],
      creatingEditionGroup: false,
    }
  },
  methods: {
    validateEditionGroup() {
      // TODO : form validation : https://primevue.org/forms/#validateon
      if (this.action == 'select') {
        // TODO: get existing editions
        this.$emit('done', { id: this.editionGroupId })
      } else {
        this.creatingEditionGroup = true
        const formattededitionGroupForm = JSON.parse(JSON.stringify(this.editionGroupForm))
        formattededitionGroupForm.tags =
          formattededitionGroupForm.tags == '' ? [] : formattededitionGroupForm.tags.split(',')
        // otherwise there is a json parse error, last char is "Z"
        formattededitionGroupForm.original_release_date =
          formattededitionGroupForm.original_release_date.slice(0, -1)
        createTitleGroup(formattededitionGroupForm).then((data) => {
          this.creatingEditionGroup = false
          this.$emit('done', data)
        })
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
    if (this.$route.query.edition_group_id) {
      this.editionGroupId = this.$route.query.edition_group_id.toString()
    }
  },
}
</script>
<style scoped>
.title {
  font-weight: bold;
  font-size: 1.5em;
  margin-bottom: 25px;
}
.title .alternative {
  font-size: 0.8em;
  color: var(--color-secondary);
  cursor: pointer;
}
.description {
  width: 100%;
  height: 10em;
}
.p-floatlabel {
  margin-bottom: 30px;
}
.select {
  width: 200px;
}
.external-db-inputs-wrapper {
  display: flex;
  align-items: center;
  margin-bottom: 55px;
}
.external-db-inputs {
  display: flex;
  align-items: center;
  margin-left: -10px;
}
.external-db-inputs .p-floatlabel {
  margin: 0px 10px;
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
