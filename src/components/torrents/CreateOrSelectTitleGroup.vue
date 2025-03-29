<template>
  <div class="title" v-if="action == 'select'">
    Select title <span class="alternative" @click="action = 'create'">(or create one)</span>
  </div>
  <div class="title" v-if="action == 'create'">
    Create a new title
    <span class="alternative" @click="action = 'select'">(or select an existing one)</span>
  </div>
  <div id="select-title-group" v-if="action == 'select'">
    <FloatLabel>
      <InputNumber size="small" v-model="titleGroupId" name="id" :format="false" />
      <label for="id">Title id</label>
    </FloatLabel>
  </div>
  <div id="create-title-group" v-if="action == 'create'">
    <FloatLabel>
      <Select
        v-model="titleGroupForm.content_type"
        inputId="content_type"
        :options="selectableContentTypes"
        class="select"
        size="small"
        @update:modelValue="(step = 2) && (manualCreation = false)"
      />
      <label for="content_type">Content type</label>
    </FloatLabel>
    <div class="external-db-inputs-wrapper" v-if="step > 1 && !manualCreation">
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'Movie'">
        <FloatLabel>
          <InputText size="small" name="tmdb_id" />
          <label for="tmdb_id">TMDB id</label>
        </FloatLabel>
        or
        <FloatLabel>
          <InputText size="small" name="imdb_id" />
          <label for="imdb_id">IMDB id</label>
        </FloatLabel>
      </div>
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'TV Show'">
        <FloatLabel>
          <InputText size="small" name="tvdb_id" />
          <label for="tvdb_id">TVDB id</label>
        </FloatLabel>
        or
        <FloatLabel>
          <InputText size="small" name="imdb_id" />
          <label for="imdb_id">IMDB id</label>
        </FloatLabel>
      </div>
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'Music'">
        <FloatLabel>
          <InputText size="small" name="musicbrainz_id" />
          <label for="musicbrainz_id">Musicbrainz id</label>
        </FloatLabel>
        or
        <FloatLabel>
          <InputText size="small" name="discogs_id" />
          <label for="discogs_id">Discogs id</label>
        </FloatLabel>
      </div>
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'Book'">
        <FloatLabel>
          <IconField>
            <InputText
              size="small"
              name="openlibrary_id"
              v-model="external_database_ids.openlibrary"
            />
            <label for="openlibrary_id">Open Library id</label>
            <InputIcon
              :class="{
                pi: true,
                'pi-search': !gettingExternalDatabaseData,
                'pi-hourglass': gettingExternalDatabaseData,
                'cursor-pointer': true,
              }"
              @click="getExternalDatabaseData(external_database_ids.openlibrary, 'openlibrary')"
            />
          </IconField>
        </FloatLabel>
      </div>
      <div v-if="step == 2">
        or
        <span
          class="cursor-pointer"
          style="margin-left: 10px; color: var(--color-secondary); font-size: 1.2em"
          @click="(step = 3) && (manualCreation = true)"
          >create the title manually</span
        >
      </div>
    </div>
    <div v-if="step > 2">
      <FloatLabel>
        <Select
          v-model="titleGroupForm.category"
          inputId="category"
          :options="selectableCategories[titleGroupForm.content_type]"
          size="small"
          class="select"
        />
        <label for="category">Category</label>
      </FloatLabel>
      <FloatLabel>
        <InputText size="small" v-model="titleGroupForm.name" name="name" />
        <label for="name">Name</label>
      </FloatLabel>
      <FloatLabel>
        <InputText size="small" v-model="titleGroupForm.tags" name="tags" />
        <label for="tags">Tags (comma separated)</label>
      </FloatLabel>
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
      <FloatLabel>
        <Select
          v-model="titleGroupForm.original_language"
          inputId="original_language"
          :options="selectableLanguages"
          class="select"
          size="small"
          filter
        />
        <label for="original_language">Original language</label>
      </FloatLabel>
      <div>
        <label for="original_release_date" class="block">Original realease date</label>
        <DatePicker
          v-model="titleGroupForm.original_release_date"
          showIcon
          :showOnFocus="false"
          inputId="original_release_date"
          size="small"
        />
      </div>
      <div class="covers input-list">
        <label>Covers</label>
        <div v-for="(link, index) in titleGroupForm.covers" :key="index">
          <InputText size="small" v-model="titleGroupForm.covers[index]" />
          <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
          <Button
            v-if="titleGroupForm.covers.length != 0"
            @click="removeCover(index)"
            icon="pi pi-minus"
            size="small"
          />
        </div>
      </div>
      <div class="external-links input-list">
        <label>External Links</label>
        <div v-for="(link, index) in titleGroupForm.external_links" :key="index">
          <InputText size="small" v-model="titleGroupForm.external_links[index]" />
          <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
          <Button
            v-if="titleGroupForm.external_links.length != 0"
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
      label="Validate title"
      @click="validateTitleGroup"
      icon="pi pi-check"
      size="small"
      class="validate-button"
      :loading="creatingTitleGroup"
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
      titleGroupForm: {
        name: '',
        description: '',
        original_language: '',
        original_release_date: null,
        content_type: '',
        covers: [''],
        external_links: [''],
        category: '',
        tags: '',
      },
      selectableLanguages: ['English', 'French'],
      selectableContentTypes: ['Movie', 'TV Show', 'Music', 'Software', 'Book'],
      selectableCategories: {
        Book: ['Illustrated', 'Periodical', 'Book', 'Article', 'Manual'],
      },
      external_database_ids: {
        openlibrary: '',
        tmdb: '',
        imdb: '',
        musicbrainz: '',
      },
      gettingExternalDatabaseData: false,
      creatingTitleGroup: false,
    }
  },
  methods: {
    getExternalDatabaseData(item_id: string | Number, database: string) {
      this.gettingExternalDatabaseData = true
      getExternalDatabaseData(item_id, database).then((data) => {
        data.title_group.original_release_date = new Date(data.title_group.original_release_date)
        this.titleGroupForm = data.title_group
        if (data.edition_group) {
          this.$emit('gotEditionData', data.edition_group)
        }
        this.step = 3
        this.gettingExternalDatabaseData = false
      })
    },
    validateTitleGroup() {
      // TODO : form validation : https://primevue.org/forms/#validateon
      if (this.action == 'select') {
        // TODO: get existing editions
        this.$emit('done', { id: this.titleGroupId })
      } else {
        this.creatingTitleGroup = true
        const formattedTitleGroupForm = JSON.parse(JSON.stringify(this.titleGroupForm))
        formattedTitleGroupForm.tags =
          formattedTitleGroupForm.tags == '' ? [] : formattedTitleGroupForm.tags.split(',')
        // otherwise there is a json parse error, last char is "Z"
        formattedTitleGroupForm.original_release_date =
          formattedTitleGroupForm.original_release_date.slice(0, -1)
        createTitleGroup(formattedTitleGroupForm).then((data) => {
          this.creatingTitleGroup = false
          this.$emit('done', data)
        })
      }
    },
    addLink() {
      this.titleGroupForm.external_links.push('')
    },
    removeLink(index: Number) {
      this.titleGroupForm.external_links.splice(index, 1)
    },
    addCover() {
      this.titleGroupForm.covers.push('')
    },
    removeCover(index: Number) {
      this.titleGroupForm.covers.splice(index, 1)
    },
  },
  created() {
    if (this.$route.query.title_group_id) {
      this.titleGroupId = this.$route.query.title_group_id.toString()
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
