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
    <Accordion value="" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>Edition group</AccordionHeader>
        <AccordionContent> Edition groupo </AccordionContent>
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

export default {
  components: {
    CreateOrSelectTitleGroup,
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionPanel,
  },
  data() {
    return {
      titleGroupAccordionValue: '0',
      titleGroup: {},
      torrentForm: {},
    }
  },
  methods: {
    titleGroupDone(titleGroup: object) {
      this.titleGroup = titleGroup
      console.log(this.titleGroupAccordionValue)
      this.titleGroupAccordionValue = ''
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
