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
            {{ getEditionGroupSlug(slotProps.option) }}
          </div>
        </template>
        <template #value="slotProps" v-if="selected_edition_group.id">
          <div>
            {{ getEditionGroupSlug(slotProps.value) }}
          </div>
        </template>
      </Select>
      <label for="edition_group">Edition</label>
    </FloatLabel>
    <div class="flex justify-content-center">
      <Button
        label="Validate edition"
        @click="sendEditionGroup"
        icon="pi pi-check"
        size="small"
        class="validate-button"
        :loading="creatingEditionGroup"
      />
    </div>
  </div>
  <div v-if="action == 'create'">
    <div v-if="step > 0">
      <CreateOrEditEditionGroup :titleGroup @validated="sendEditionGroup" />
    </div>
  </div>
</template>

<script lang="ts">
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { createEditionGroup } from '@/services/api/torrentService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { getEditionGroupSlug } from '@/services/helpers'
import CreateOrEditEditionGroup from './CreateOrEditEditionGroup.vue'

export default {
  components: {
    Button,
    FloatLabel,
    Select,
    CreateOrEditEditionGroup,
  },
  data() {
    return {
      action: 'select', // create | select
      step: 1,
      manualCreation: false,
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
    sendEditionGroup(editionGroupForm) {
      if (this.action == 'select') {
        this.$emit('done', this.selected_edition_group)
      } else {
        this.creatingEditionGroup = true
        const formattededitionGroupForm = JSON.parse(JSON.stringify(editionGroupForm))
        // otherwise there is a json parse error, last char is "Z"
        formattededitionGroupForm.release_date = formattededitionGroupForm.release_date.slice(0, -1)
        createEditionGroup(formattededitionGroupForm).then((data) => {
          this.creatingEditionGroup = false
          this.$emit('done', data)
        })
      }
    },
  },
  created() {
    const titleGroupStore = useTitleGroupStore()
    if (titleGroupStore.id) {
      this.titleGroup = titleGroupStore
    }
  },
}
</script>
<style scoped>
.title {
  font-weight: bold;
  font-size: 1.5em;
}
.title .alternative {
  font-size: 0.8em;
  color: var(--color-secondary);
  cursor: pointer;
}
.p-floatlabel {
  margin: 30px 0px;
}
.select-existing-edition {
  width: 500px;
}
.validate-button {
  margin-top: 20px;
}
</style>
