<template>
  <div id="create-torrent">
    <FloatLabel>
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
      <FloatLabel>
        <InputText v-model="torrentForm.release_name" size="small" name="release_name" />
        <label for="release_name">Release name</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="torrentForm.release_group" size="small" name="release_group" />
        <label for="release_group">Release group</label>
      </FloatLabel>
      <FloatLabel>
        <Textarea
          v-model="torrentForm.description"
          name="description"
          class="textarea"
          autoResize
          rows="5"
        />
        <label for="description">Description</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="torrentForm.container" size="small" name="container" />
        <label for="container">Container</label>
      </FloatLabel>
      <FloatLabel>
        <Select
          v-model="torrentForm.video_codec"
          inputId="video_coded"
          :options="selectableVideoCodecs"
          class="select"
          size="small"
        />
        <label for="video_coded">Video codec</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="torrentForm.duration" size="small" name="duration" />
        <label for="duration">Duration (total, in seconds)</label>
      </FloatLabel>
      <FloatLabel>
        <Select
          v-model="torrentForm.audio_codec"
          inputId="audio_coded"
          :options="selectableAudioCodecs"
          class="select"
          size="small"
        />
        <label for="audio_coded">Audio codec</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="torrentForm.audio_bitrate" size="small" name="audio_bitrate" />
        <label for="audio_codec">Audio bitrate (in kb/s)</label>
      </FloatLabel>
      <FileUpload ref="torrentFile" accept="*" chooseLabel="Torrent file"> </FileUpload>
      <div class="flex align-items-center">
        <Checkbox v-model="torrentForm.uploaded_as_anonymous" name="anonymous" binary />
        <label for="anonymous" style="margin-left: 5px"> Upload as anonymous</label>
      </div>
    </div>
  </div>
  <div class="flex justify-content-center">
    <Button
      label="Validate torrent"
      @click="validateTorrent"
      icon="pi pi-check"
      size="small"
      class="validate-button"
      :loading="creatingTorrent"
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
import InputIcon from 'primevue/inputicon'
import IconField from 'primevue/iconfield'
import Checkbox from 'primevue/checkbox'
import FileUpload from 'primevue/fileupload'
import { getFileInfo } from '@/services/fileinfo/fileinfo.js'

export default {
  components: {
    // eslint-disable-next-line vue/no-reserved-component-names
    Button,
    FileUpload,
    InputNumber,
    FloatLabel,
    InputText,
    // eslint-disable-next-line vue/no-reserved-component-names
    Textarea,
    // eslint-disable-next-line vue/no-reserved-component-names
    Select,
    InputIcon,
    IconField,
    Checkbox,
  },
  props: {},
  data() {
    return {
      step: 1,
      torrentForm: {
        release_name: '',
        release_group: '',
        mediainfo: '',
        description: '',
        uploaded_as_anonymous: false,
      },
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
      creatingTorrent: false,
    }
  },
  methods: {
    mediainfoUpdated() {
      const mediainfoExtractedInfo = getFileInfo(this.torrentForm.mediainfo)
      console.log(mediainfoExtractedInfo)
      this.step = 2
    },
    validateTorrent() {
      // TODO : form validation : https://primevue.org/forms/#validateon

      this.creatingTorrent = true
      // const formattedTitleGroupForm = JSON.parse(JSON.stringify(this.titleGroupForm))
      // formattedTitleGroupForm.tags =
      //   formattedTitleGroupForm.tags == '' ? [] : formattedTitleGroupForm.tags.split(',')
      // // otherwise there is a json parse error, last char is "Z"
      // formattedTitleGroupForm.original_release_date =
      //   formattedTitleGroupForm.original_release_date.slice(0, -1)
      // createTitleGroup(formattedTitleGroupForm).then((data) => {
      //   this.creatingTitleGroup = false
      //   this.$emit('done', data)
      // })
    },
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
.select {
  width: 200px;
}
.validate-button {
  margin-top: 20px;
}
</style>
