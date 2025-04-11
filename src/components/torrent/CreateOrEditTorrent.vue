<template>
  <Form
    v-slot="$form"
    :initialValues="torrentForm"
    :resolver
    @submit="onFormSubmit"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
  >
    <div id="create-torrent">
      <FloatLabel>
        <Message v-if="$form.mediainfo?.invalid" severity="error" size="small" variant="simple">
          {{ $form.mediainfo.error?.message }}
        </Message>
        <Textarea
          v-model="torrentForm.mediainfo"
          name="mediainfo"
          class="textarea pre-style"
          rows="5"
          @update:model-value="mediainfoUpdated"
        />
        <label for="mediainfo">Mediainfo</label>
      </FloatLabel>
      <div v-if="step > 1">
        <div class="line">
          <FloatLabel class="release-name">
            <Message
              v-if="$form.release_name?.invalid"
              severity="error"
              size="small"
              variant="simple"
            >
              {{ $form.release_name.error?.message }}
            </Message>
            <InputText v-model="torrentForm.release_name" size="small" name="release_name" />
            <label for="release_name">Release name</label>
          </FloatLabel>
          <FloatLabel>
            <Message
              v-if="$form.release_group?.invalid"
              severity="error"
              size="small"
              variant="simple"
            >
              {{ $form.release_name.error?.message }}
            </Message>
            <InputText v-model="torrentForm.release_group" size="small" name="release_group" />
            <label for="release_group">Release group</label>
          </FloatLabel>
        </div>
        <FloatLabel>
          <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
            {{ $form.release_name.error?.message }}
          </Message>
          <Textarea
            v-model="torrentForm.description"
            name="description"
            class="textarea"
            autoResize
            rows="5"
          />
          <label for="description">Description</label>
        </FloatLabel>

        <div class="line">
          <FloatLabel>
            <Message v-if="$form.container?.invalid" severity="error" size="small" variant="simple">
              {{ $form.container.error?.message }}
            </Message>
            <InputText v-model="torrentForm.container" size="small" name="container" />
            <label for="container">Container</label>
          </FloatLabel>
          <FloatLabel>
            <Message
              v-if="$form.video_codec?.invalid"
              severity="error"
              size="small"
              variant="simple"
            >
              {{ $form.video_codec.error?.message }}
            </Message>
            <Select
              v-model="torrentForm.video_codec"
              inputId="video_coded"
              :options="selectableVideoCodecs"
              class="select"
              size="small"
              name="video_codec"
            />
            <label for="video_coded">Video codec</label>
          </FloatLabel>
          <FloatLabel>
            <Message
              v-if="$form.video_resolution?.invalid"
              severity="error"
              size="small"
              variant="simple"
            >
              {{ $form.video_codec.error?.message }}
            </Message>
            <Select
              v-model="torrentForm.video_resolution"
              inputId="video_resolution"
              :options="selectableVideoResolutions"
              class="select"
              size="small"
              name="video_resolution"
            />
            <label for="video_resolution">Video resolution</label>
          </FloatLabel>
          <FloatLabel>
            <Message
              v-if="$form.audio_codec?.invalid"
              severity="error"
              size="small"
              variant="simple"
            >
              {{ $form.video_codec.error?.message }}
            </Message>
            <Select
              v-model="torrentForm.audio_codec"
              inputId="audio_coded"
              :options="selectableAudioCodecs"
              class="select"
              size="small"
              name="audio_codec"
            />
            <label for="audio_coded">Audio codec</label>
          </FloatLabel>
        </div>
        <FloatLabel>
          <Message v-if="$form.language?.invalid" severity="error" size="small" variant="simple">
            {{ $form.language.error?.message }}
          </Message>
          <Select
            v-model="torrentForm.language"
            inputId="language"
            :options="selectableLanguages"
            class="select"
            size="small"
            name="language"
          />
          <label for="language">Language</label>
        </FloatLabel>
        <FloatLabel>
          <MultiSelect
            v-model="torrentForm.features"
            display="chip"
            :options="selectableFeatures"
            filter
            size="small"
            name="features"
          />
          <label for="features">Features</label>
        </FloatLabel>
        <FloatLabel>
          <InputText v-model="torrentForm.duration" size="small" name="duration" />
          <label for="duration">Duration (total, in seconds)</label>
        </FloatLabel>
        <!-- <FloatLabel>
        <InputText v-model="torrentForm.audio_bitrate" size="small" name="audio_bitrate" />
        <label for="audio_codec">Audio bitrate (in kb/s)</label>
      </FloatLabel> -->
        <FormField v-slot="$field" name="torrent_file" :initialValue="torrentForm.torrent_file">
          <Message
            v-if="$form.torrent_file?.invalid"
            severity="error"
            size="small"
            variant="simple"
          >
            {{ $form.torrent_file.error?.message }}
          </Message>
          <FileUpload
            ref="torrentFile"
            accept="application/x-bittorrent"
            chooseLabel="Torrent file"
            :showCancelButton="false"
            :showUploadButton="false"
            @select="onFileSelect"
            v-bind="$field.props"
          >
            <template #content="{ files }">{{
              files.length != 0 ? files[0].name : 'Select a file'
            }}</template>
          </FileUpload>
        </FormField>
        <div class="flex align-items-center">
          <Checkbox v-model="torrentForm.uploaded_as_anonymous" name="anonymous" binary />
          <label for="anonymous" style="margin-left: 5px"> Upload as anonymous</label>
        </div>
      </div>
      <div class="flex justify-content-center">
        <Button
          label="Validate torrent"
          type="submit"
          icon="pi pi-check"
          size="small"
          class="validate-button"
          :loading="uploadingTorrent"
        />
      </div>
    </div>
  </Form>
</template>

<script lang="ts">
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import FileUpload from 'primevue/fileupload'
import MultiSelect from 'primevue/multiselect'
import Message from 'primevue/message'
import { FormField } from '@primevue/forms'
import { Form } from '@primevue/forms'
import { getFileInfo } from '@/services/fileinfo/fileinfo.js'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { uploadTorrent } from '@/services/api/torrentService'
import { getFeatures } from '@/services/helpers'
import { useTitleGroupStore } from '@/stores/titleGroup'

export default {
  components: {
    Form,
    Message,
    FormField,
    Button,
    MultiSelect,
    FileUpload,
    FloatLabel,
    InputText,
    Textarea,
    Select,
    Checkbox,
  },
  props: {},
  data() {
    return {
      step: 1,
      torrentForm: {
        edition_group_id: '',
        release_name: '',
        release_group: '',
        mediainfo: '',
        description: '',
        language: '',
        container: '',
        video_codec: '',
        video_resolution: '',
        duration: null,
        audio_codec: '',
        audio_bitrate: null,
        subtitle_languages: '',
        features: '',
        audio_channels: '',
        audio_bitrate_sampling: null,
        torrent_file: null,
        uploaded_as_anonymous: false,
      },
      // TODO : move all the selectable* arrays to an helper function
      selectableVideoCodecs: [
        'Mpeg1',
        'Mpeg2',
        'Xvid',
        'DivX',
        'H264',
        'H265',
        'Vc1',
        'Vp9',
        'BD50',
        'UHD100',
      ],
      selectableVideoResolutions: ['2160p', '1440p', '1080p', '720p', 'SD'],
      selectableAudioCodecs: [
        'Aac',
        'Opus',
        'Mp3',
        'Mp2',
        'Aac',
        'Ac3',
        'Dts',
        'Flac',
        'Pcm',
        'TrueHd',
        'Dsd',
      ],
      selectableLanguages: ['English', 'French'],
      uploadingTorrent: false,
      content_type: '',
    }
  },
  methods: {
    resolver({ values }) {
      const errors = {}

      if (values.release_name.length < 5) {
        errors.release_name = [{ message: 'Write more than 5 characters' }]
      }
      if (values.release_group.length < 2) {
        errors.release_group = [{ message: 'Write more than 2 characters' }]
      }
      // if (values.description == '') {
      //   errors.description = [{ message: 'Write a description' }]
      // }
      if (values.container == '') {
        errors.container = [{ message: 'File container missing' }]
      }
      if (values.video_codec == '') {
        errors.video_codec = [{ message: 'Select a codec' }]
      }
      if (values.video_resolution == '') {
        errors.video_resolution = [{ message: 'Select a resolution' }]
      }
      if (values.audio_codec == '') {
        errors.audio_codec = [{ message: 'Select a codec' }]
      }
      if (values.language == '') {
        errors.language = [{ message: 'Select a language' }]
      }
      if (!this.torrentForm.torrent_file) {
        errors.torrent_file = [{ message: 'Select a torrent_file' }]
      }
      console.log(errors)

      return {
        errors,
      }
    },
    onFormSubmit({ valid }) {
      if (valid) {
        this.sendTorrent()
      }
    },
    onFileSelect(event) {
      if (event.files && event.files.length > 0) {
        // keep a single file
        const torrentFile = event.files[event.files.length - 1]
        this.$refs.torrentFile.clear()
        this.$refs.torrentFile.files = [torrentFile]
        this.torrentForm.torrent_file = torrentFile
      }
    },
    mediainfoUpdated() {
      const mediainfoExtractedInfo = getFileInfo(this.torrentForm.mediainfo)
      console.log(mediainfoExtractedInfo)
      this.step = 2
    },
    sendTorrent() {
      this.uploadingTorrent = true
      // const formattedTitleGroupForm = JSON.parse(JSON.stringify(this.titleGroupForm))
      // formattedTitleGroupForm.tags =
      //   formattedTitleGroupForm.tags == '' ? [] : formattedTitleGroupForm.tags.split(',')
      // // otherwise there is a json parse error, last char is "Z"
      // formattedTitleGroupForm.original_release_date =
      //   formattedTitleGroupForm.original_release_date.slice(0, -1)
      uploadTorrent(this.torrentForm).then((data) => {
        this.uploadingTorrent = false
        this.$emit('done', data)
      })
    },
  },
  computed: {
    selectableFeatures() {
      return getFeatures(this.content_type)
    },
  },
  created() {
    this.content_type = useTitleGroupStore().content_type
    const editionGroupStore = useEditionGroupStore()
    this.torrentForm.edition_group_id = editionGroupStore.id
  },
}
</script>
<style scoped>
.textarea {
  width: 100%;
  height: 10em;
}
.p-floatlabel {
  margin-bottom: 30px;
}
.release-name {
  width: 100%;
  input {
    width: 100%;
  }
}
.select {
  width: 200px;
}
.file-upload {
  max-width: 300px;
}
.validate-button {
  margin-top: 20px;
}
</style>
<style>
#create-torrent {
  .p-fileupload {
    max-width: 300px;
    margin-bottom: 15px;
  }
}
</style>
