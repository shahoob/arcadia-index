<template>
  <div>
    <div class="title">Upload torrent</div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>Title</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectTitleGroup @done="titleGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="editionGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep < 2">
        <AccordionHeader>Edition</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectEditionGroup v-if="uploadStep > 1" @done="editionGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="torrentAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep < 3">
        <AccordionHeader>Torrent</AccordionHeader>
        <AccordionContent><CreateTorrent v-if="uploadStep > 2" /></AccordionContent>
      </AccordionPanel>
    </Accordion>
  </div>
</template>
<script lang="ts">
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import CreateOrSelectTitleGroup from '@/components/torrents/CreateOrSelectTitleGroup.vue'
import CreateOrSelectEditionGroup from '@/components/torrents/CreateOrSelectEditionGroup.vue'
import CreateTorrent from '@/components/torrents/CreateTorrent.vue'
import { useEditionGroupStore } from '@/stores/editionGroup'

export default {
  components: {
    CreateOrSelectTitleGroup,
    CreateOrSelectEditionGroup,
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionPanel,
    CreateTorrent,
  },
  data() {
    return {
      titleGroupAccordionValue: '0',
      editionGroupAccordionValue: '',
      torrentAccordionValue: '',
      uploadStep: 1,
      editionGroup: {},
    }
  },
  created() {},
  methods: {
    titleGroupDone() {
      this.titleGroupAccordionValue = ''
      this.editionGroupAccordionValue = '0'
      this.uploadStep = 2
    },
    editionGroupDone(editionGroup: object) {
      this.editionGroup = editionGroup
      const editionGroupStore = useEditionGroupStore()
      editionGroupStore.id = editionGroup.id
      this.editionGroupAccordionValue = ''
      this.torrentAccordionValue = '0'
      this.uploadStep = 3
    },
  },
}
</script>

<style scoped>
.title {
  font-weight: bold;
  font-size: 1.8em;
  margin-bottom: 20px;
  color: var(--color-primary);
}
.upload-step-accordion {
  margin-bottom: 20px;
}
</style>
