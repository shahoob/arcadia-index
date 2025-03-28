<template>
  <div>
    <div class="title">Upload torrent</div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>Title group</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectTitleGroup @done="titleGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="editionGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep < 2">
        <AccordionHeader>Edition group</AccordionHeader>
        <AccordionContent> <CreateOrSelectEditionGroup /> </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion value="" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep < 3">
        <AccordionHeader>Torrent</AccordionHeader>
        <AccordionContent>Torrent</AccordionContent>
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

export default {
  components: {
    CreateOrSelectTitleGroup,
    CreateOrSelectEditionGroup,
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionPanel,
  },
  data() {
    return {
      titleGroupAccordionValue: '0',
      editionGroupAccordionValue: '',
      uploadStep: 1,
      titleGroup: {},
      torrentForm: {},
    }
  },
  methods: {
    titleGroupDone(titleGroup: object) {
      this.titleGroup = titleGroup
      this.titleGroupAccordionValue = ''
      this.editionGroupAccordionValue = '0'
      this.uploadStep = 2
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
