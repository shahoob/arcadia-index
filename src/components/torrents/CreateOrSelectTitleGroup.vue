<template>
  <ContentContainer>
    <div class="title" v-if="action == 'select'">
      Select title group <span class="alternative" @click="action = 'create'">(or create one)</span>
    </div>
    <div class="title" v-if="action == 'create'">
      Create a new title group
      <span class="alternative" @click="action = 'select'">(or select an existing one)</span>
    </div>
    <div id="select-title-group" v-if="action == 'select'">
      <FloatLabel>
        <InputNumber name="id" :format="false" />
        <label for="id">Title group id</label>
      </FloatLabel>
    </div>
    <div id="create-title-group" v-if="action == 'create'">
      <div>
        <FloatLabel>
          <Select
            v-model="createForm.content_type"
            inputId="content_type"
            :options="selectableContentTypes"
            optionLabel="name"
            class="select"
            @update:modelValue="step = 2"
          />
          <label for="content_type">Content type</label>
        </FloatLabel>
      </div>
      <div class="external_db_inputs_wrapper" v-if="step > 1 && !manualCreation">
        <div class="external_db_inputs" v-if="createForm.content_type.name == 'Movie'">
          <FloatLabel>
            <InputText name="tmdb_id" />
            <label for="tmdb_id">TMDB id</label>
          </FloatLabel>
          or
          <FloatLabel>
            <InputText name="imdb_id" />
            <label for="imdb_id">IMDB id</label>
          </FloatLabel>
        </div>
        <div class="external_db_inputs" v-if="createForm.content_type.name == 'TV Show'">
          <FloatLabel>
            <InputText name="tvdb_id" />
            <label for="tvdb_id">TVDB id</label>
          </FloatLabel>
          or
          <FloatLabel>
            <InputText name="imdb_id" />
            <label for="imdb_id">IMDB id</label>
          </FloatLabel>
        </div>
        <div class="external_db_inputs" v-if="createForm.content_type.name == 'Music'">
          <FloatLabel>
            <InputText name="musicbrainz_id" />
            <label for="musicbrainz_id">Musicbrainz id</label>
          </FloatLabel>
          or
          <FloatLabel>
            <InputText name="discogs_id" />
            <label for="discogs_id">Discogs id</label>
          </FloatLabel>
        </div>
        <div class="external_db_inputs" v-if="createForm.content_type.name == 'Book'">
          <FloatLabel>
            <IconField>
              <InputText name="openlibrary_id" v-model="external_database_ids.openlibrary" />
              <label for="openlibrary_id">Open Library id</label>
              <InputIcon
                class="pi pi-search cursor-pointer"
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
            >create the title group manually</span
          >
        </div>
      </div>
      <div v-if="step > 2">
        <FloatLabel>
          <InputText v-model="createForm.name" name="name" />
          <label for="name">Name</label>
        </FloatLabel>
        <FloatLabel>
          <Textarea
            v-model="createForm.description"
            name="description"
            class="description"
            autoResize
            rows="5"
          />
          <label for="description">Description</label>
        </FloatLabel>
        <FloatLabel>
          <Select
            v-model="createForm.original_language"
            inputId="original_language"
            :options="selectableLanguages"
            optionLabel="name"
            class="select"
            filter
          />
          <label for="original_language">Original language</label>
        </FloatLabel>
        <div class="external_links">
          <label>External Links</label>
          <div v-for="(link, index) in createForm.external_links" :key="index">
            <InputText :id="`external-link-${index}`" v-model="createForm.external_links[index]" />
            <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
            <Button
              v-if="createForm.external_links.length != 0"
              @click="removeLink(index)"
              icon="pi pi-minus"
              size="small"
            />
          </div>
        </div>
      </div>
    </div>
  </ContentContainer>
</template>

<script lang="ts">
import { InputNumber } from 'primevue'
import FloatLabel from 'primevue/floatlabel'
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { getExternalDatabaseData } from '@/services/api/externalDatabasesService'
import InputIcon from 'primevue/inputicon'
import IconField from 'primevue/iconfield'
import { create } from 'domain'

export default {
  // eslint-disable-next-line vue/no-reserved-component-names
  components: {
    Button,
    InputNumber,
    FloatLabel,
    ContentContainer,
    InputText,
    Textarea,
    Select,
    InputIcon,
    IconField,
  },
  props: {},
  data() {
    return {
      action: 'create', // create | select
      step: 1,
      manualCreation: false,
      createForm: { description: '', original_language: '', content_type: '', external_links: [] },
      selectableLanguages: [{ name: 'English' }, { name: 'French' }],
      selectableContentTypes: [
        { name: 'Movie' },
        { name: 'TV Show' },
        { name: 'Music' },
        { name: 'Software' },
        { name: 'Book' },
      ],
      external_database_ids: {
        openlibrary: '',
        tmdb: '',
        imdb: '',
        musicbrainz: '',
      },
    }
  },
  methods: {
    getExternalDatabaseData(item_id: string | Number, database: string) {
      getExternalDatabaseData(item_id, database).then((data) => {
        data.content_type = { name: data.content_type }
        this.createForm = data
        this.step = 3
      })
    },
    addLink() {
      this.createForm.external_links.push('')
    },
    removeLink(index: Number) {
      this.createForm.external_links.splice(index, 1)
    },
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
.external_db_inputs_wrapper {
  display: flex;
  align-items: center;
  margin-bottom: 55px;
}
.external_db_inputs {
  display: flex;
  align-items: center;
  margin-left: -10px;
}
.external_db_inputs .p-floatlabel {
  margin: 0px 10px;
}
.external_links .p-component {
  margin-right: 5px;
  margin-bottom: 5px;
}
.external_links input {
  width: 400px;
}
</style>
