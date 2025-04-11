<template>
  <div class="title" v-if="action == 'select'">
    Select edition <span class="alternative" @click="action = 'create'">(or create one)</span>
  </div>
  <div class="title" v-if="action == 'create'">
    Create a new edition
    <span class="alternative" @click="action = 'select'">(or select an existing one)</span>
  </div>
  <div id="select-edition-group" v-if="action == 'select'">
    <FloatLabel>
      <Select
        v-model="selected_edition_group"
        inputId="edition_group"
        :options="titleGroup.edition_groups"
        size="small"
        class="select-existing-edition"
      >
        <template #option="slotProps">
          <div>
            <!-- {{ slotProps.option.name }} / {{ slotProps.option.source }} /
            {{ slotProps.option.release_date.substring(0, 10) }} -->
            {{ getEditionGroupSlug(slotProps.option) }}
          </div>
        </template>
        <template #value="slotProps" v-if="selected_edition_group.id">
          <div>
            <!-- {{ slotProps.value.name }} / {{ slotProps.value.source }} /
            {{ slotProps.value.release_date.substring(0, 10) }} -->
            {{ getEditionGroupSlug(slotProps.value) }}
          </div>
        </template>
      </Select>
      <label for="edition_group">Edition</label>
    </FloatLabel>
  </div>
  <div v-if="action == 'create'">
    <div v-if="step > 0">
      <div class="line">
        <FloatLabel>
          <InputText size="small" v-model="editionGroupForm.name" name="name" />
          <label for="name">Name</label>
        </FloatLabel>
        <FloatLabel>
          <InputText size="small" v-model="editionGroupForm.distributor" name="distributor" />
          <label for="distributor">Distributor</label>
        </FloatLabel>
        <FloatLabel>
          <Select
            v-model="editionGroupForm.source"
            inputId="source"
            :options="selectableSources[titleGroup.content_type]"
            class="select-source"
            size="small"
          />
          <label for="source">Source</label>
        </FloatLabel>
      </div>
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
      <div>
        <label for="release_date" class="block">Realease date</label>
        <DatePicker
          v-model="editionGroupForm.release_date"
          showIcon
          :showOnFocus="false"
          inputId="release_date"
          size="small"
          dateFormat="yy-mm-dd"
        />
      </div>
      <div class="covers input-list">
        <label>Covers</label>
        <div v-for="(link, index) in editionGroupForm.covers" :key="index">
          <InputText size="small" v-model="editionGroupForm.covers[index]" />
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
          <InputText size="small" v-model="editionGroupForm.external_links[index]" />
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
      label="Validate edition"
      @click="validateEditionGroup"
      icon="pi pi-check"
      size="small"
      class="validate-button"
      :loading="creatingEditionGroup"
    />
  </div>
</template>

<script lang="ts">
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import { createEditionGroup, getTitleGroupLite } from '@/services/api/torrentService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { getEditionGroupSlug } from '@/services/helpers'

export default {
  components: {
    DatePicker,
    Button,
    FloatLabel,
    InputText,
    Textarea,
    Select,
  },
  data() {
    return {
      action: 'select', // create | select
      step: 1,
      manualCreation: false,
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
      titleGroup: { edition_groups: [] },
      selected_edition_group: {},
      editionGroupId: '',
      creatingEditionGroup: false,
    }
  },
  methods: {
    getEditionGroupSlug(editionGroup) {
      return getEditionGroupSlug(editionGroup)
    },
    validateEditionGroup() {
      if (this.action == 'select') {
        this.$emit('done', this.selected_edition_group)
      } else {
        this.creatingEditionGroup = true
        const formattededitionGroupForm = JSON.parse(JSON.stringify(this.editionGroupForm))
        // otherwise there is a json parse error, last char is "Z"
        formattededitionGroupForm.release_date = formattededitionGroupForm.release_date.slice(0, -1)
        createEditionGroup(formattededitionGroupForm).then((data) => {
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
    const titleGroupStore = useTitleGroupStore()
    if (titleGroupStore.id) {
      this.titleGroup = titleGroupStore
      this.editionGroupForm.title_group_id = titleGroupStore.id
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
.select-existing-edition {
  width: 500px;
}
.select-source {
  width: 150px;
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
