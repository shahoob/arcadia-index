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
        or
        <span
          class="cursor-pointer"
          style="margin-left: 10px; color: var(--color-secondary); font-size: 1.2em"
          @click="(step = 3) && (manualCreation = true)"
          >create the title group manually</span
        >
      </div>
      <div v-if="step > 2">
        <FloatLabel>
          <InputText name="name" />
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

export default {
  components: { InputNumber, FloatLabel, ContentContainer, InputText, Textarea, Select },
  props: {},
  data() {
    return {
      action: 'create', // create | select
      step: 1,
      manualCreation: false,
      createForm: { description: '', original_language: '', content_type: '' },
      selectableLanguages: [{ name: 'English' }, { name: 'French' }],
      selectableContentTypes: [
        { name: 'Movie' },
        { name: 'TV Show' },
        { name: 'Music' },
        { name: 'Software' },
        { name: 'Book' },
      ],
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
  width: 50%;
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
}
.external_db_inputs {
  display: flex;
  align-items: center;
  margin-left: -10px;
}
.external_db_inputs .p-floatlabel {
  margin: 0px 10px;
}
</style>
