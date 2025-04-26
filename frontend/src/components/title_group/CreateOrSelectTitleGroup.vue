<template>
  <div class="title" v-if="action == 'select'">
    {{ $t('title_group.select_title') }}
    <span class="alternative" @click="action = 'create'">({{ $t('general.or_create_one') }})</span>
  </div>
  <div class="title" v-if="action == 'create'">
    {{ $t('title_group.create_title') }}
    <span class="alternative" @click="action = 'select'">({{ $t('general.or_select_one') }})</span>
  </div>

  <div id="select-title-group" v-if="action == 'select'">
    <FloatLabel>
      <InputNumber size="small" v-model="titleGroupId" name="id" :format="false" />
      <label for="id">{{ $t('title_group.title_group_id') }}</label>
    </FloatLabel>
    <div class="flex justify-content-center">
      <Button
        v-if="step == 3 || action == 'select'"
        :label="$t('general.submit')"
        @click="sendTitleGroup"
        icon="pi pi-check"
        size="small"
        class="validate-button"
        :loading="gettingTitleGroupInfo"
      />
    </div>
  </div>
  <div id="create-title-group" v-if="action == 'create'">
    <FloatLabel>
      <Select
        v-model="content_type"
        inputId="content_type"
        :options="selectableContentTypes"
        class="select"
        size="small"
        @update:modelValue="(step = 2) && (manualCreation = false)"
      >
        <template #option="slotProps">
          <span>{{ $t(`title_group.content_type.${slotProps.option}`) }}</span>
        </template>
        <template #value="slotProps">
          <span v-if="slotProps.value">
            {{ $t(`title_group.content_type.${slotProps.value}`) }}
          </span>
        </template>
      </Select>
      <label for="content_type">{{ $t('title_group.content_type.content_type') }}</label>
    </FloatLabel>
    <div class="external-db-inputs-wrapper" v-if="step > 1 && !manualCreation">
      <div class="external-db-inputs" v-if="content_type == 'movie'">
        <FloatLabel>
          <IconField>
            <InputText size="small" name="tmdb_id" v-model="external_database_ids.tmdb" />
            <label for="tmdb_id">TMDB id</label>
            <InputIcon
              :class="{
                pi: true,
                'pi-search': !gettingExternalDatabaseData,
                'pi-hourglass': gettingExternalDatabaseData,
                'pi-spin': gettingExternalDatabaseData,
                'cursor-pointer': true,
              }"
              @click="getExternalDatabaseData(external_database_ids.tmdb, 'tmdb/movie')"
            />
          </IconField>
        </FloatLabel>
        or
        <FloatLabel>
          <InputText size="small" name="imdb_id" />
          <label for="imdb_id">IMDB id</label>
        </FloatLabel>
      </div>
      <div class="external-db-inputs" v-if="content_type == 'tv_show'">
        <FloatLabel>
          <InputText size="small" name="tvdb_id" />
          <label for="tvdb_id">TVDB id</label>
        </FloatLabel>
        or
        <FloatLabel>
          <IconField>
            <InputText size="small" name="tmdb_id" v-model="external_database_ids.tmdb" />
            <label for="tmdb_id">TMDB id</label>
            <InputIcon
              :class="{
                pi: true,
                'pi-search': !gettingExternalDatabaseData,
                'pi-hourglass': gettingExternalDatabaseData,
                'pi-spin': gettingExternalDatabaseData,
                'cursor-pointer': true,
              }"
              @click="getExternalDatabaseData(external_database_ids.tmdb, 'tmdb/tv')"
            />
          </IconField>
        </FloatLabel>
        or
        <FloatLabel>
          <InputText size="small" name="imdb_id" />
          <label for="imdb_id">IMDB id</label>
        </FloatLabel>
      </div>
      <div class="external-db-inputs" v-if="content_type == 'music'">
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
      <div class="external-db-inputs" v-if="content_type == 'book'">
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
                'pi-spin': gettingExternalDatabaseData,

                'cursor-pointer': true,
              }"
              @click="getExternalDatabaseData(external_database_ids.openlibrary, 'openlibrary')"
            />
          </IconField>
        </FloatLabel>
      </div>
      <div v-if="step == 2" class="external-db-inputs" style="margin-left: 5px">
        or
        <span
          class="cursor-pointer"
          style="margin-left: 10px; color: var(--color-secondary); font-size: 1.2em"
          @click="(step = 3) && (manualCreation = true)"
        >
          create the title manually
        </span>
      </div>
    </div>
    <div v-if="step > 2">
      <CreateOrEditTitleGroup
        :content_type="content_type"
        @validated="sendTitleGroup"
        :initialTitleGroupForm
        :sendingTitleGroup
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { InputNumber } from 'primevue'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { getExternalDatabaseData } from '@/services/api/externalDatabasesService'
import InputIcon from 'primevue/inputicon'
import IconField from 'primevue/iconfield'
import { createTitleGroup, getTitleGroupLite } from '@/services/api/torrentService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import CreateOrEditTitleGroup from '../title_group/CreateOrEditTitleGroup.vue'

export default defineComponent({
  components: {
    CreateOrEditTitleGroup,
    Button,
    InputNumber,
    FloatLabel,
    InputText,
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
      selectableContentTypes: ['movie', 'tv_show', 'music', 'software', 'book', 'collection'],
      content_type: '',
      gettingTitleGroupInfo: false,
      sendingTitleGroup: false,
      initialTitleGroupForm: {},
      external_database_ids: {
        openlibrary: '',
        tmdb: '',
        imdb: '',
        musicbrainz: '',
      },
      gettingExternalDatabaseData: false,
    }
  },
  setup() {
    const titleGroupStore = useTitleGroupStore()
    // const createOrEditTitleGroupRef = ref(null)
    return { titleGroupStore }
  },
  methods: {
    getExternalDatabaseData(item_id: string | number, database: string) {
      this.gettingExternalDatabaseData = true
      getExternalDatabaseData(item_id, database).then((data) => {
        data.title_group.original_release_date = new Date(data.title_group.original_release_date)
        this.initialTitleGroupForm = data.title_group
        // if the api also returns some info about the specific edition, pass it to the edition component
        if (data.edition_group) {
          this.$emit('gotEditionData', data.edition_group)
        }
        this.step = 3
        this.gettingExternalDatabaseData = false
      })
    },
    async sendTitleGroup(titleGroupForm: object) {
      if (this.action == 'select') {
        this.gettingTitleGroupInfo = true
        if (!this.titleGroupStore.id) {
          const titleGroupLite = await getTitleGroupLite(this.titleGroupId)
          this.titleGroupStore.id = titleGroupLite.id
          this.titleGroupStore.edition_groups = titleGroupLite.edition_groups
          this.titleGroupStore.content_type = titleGroupLite.content_type
        }
        this.$emit('done')
        this.gettingTitleGroupInfo = false
      } else {
        this.sendingTitleGroup = true
        titleGroupForm.content_type = this.content_type
        const formattedTitleGroupForm = JSON.parse(JSON.stringify(titleGroupForm))
        formattedTitleGroupForm.tags =
          formattedTitleGroupForm.tags == '' ? [] : formattedTitleGroupForm.tags.split(',')
        // otherwise there is a json parse error, last char is "Z"
        formattedTitleGroupForm.original_release_date =
          formattedTitleGroupForm.original_release_date.slice(0, -1)
        createTitleGroup(formattedTitleGroupForm).then((data) => {
          // this.creatingTitleGroup = false
          this.sendingTitleGroup = false
          this.titleGroupStore.id = data.id
          this.titleGroupStore.content_type = data.content_type
          this.$emit('done', data)
        })
      }
    },
  },
  created() {
    if (this.titleGroupStore.id) {
      this.titleGroupId = this.titleGroupStore.id.toString()
    }
  },
})
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
.p-floatlabel {
  margin-top: 30px;
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
  margin-top: 30px;
}
.external-db-inputs .p-floatlabel {
  margin: 0px 10px;
}
.validate-button {
  margin-top: 20px;
}
</style>
