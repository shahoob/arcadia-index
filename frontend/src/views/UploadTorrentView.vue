<template>
  <div>
    <div class="title">{{ t('torrent.upload_torrent') }}</div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep != 1">
        <AccordionHeader>{{ t('title_group.title') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectTitleGroup @done="titleGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="editionGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep != 2">
        <AccordionHeader>{{ t('torrent.edition') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectEditionGroup v-if="uploadStep > 1" @done="editionGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="torrentAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="uploadStep != 3">
        <AccordionHeader>{{ t('torrent.torrent') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrEditTorrent v-if="uploadStep > 2" @done="torrentDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import CreateOrSelectTitleGroup from '@/components/title_group/CreateOrSelectTitleGroup.vue'
import CreateOrSelectEditionGroup from '@/components/edition_group/CreateOrSelectEditionGroup.vue'
import CreateOrEditTorrent from '@/components/torrent/CreateOrEditTorrent.vue'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const { t } = useI18n()

const editionGroupStore = useEditionGroupStore()
const titleGroupStore = useTitleGroupStore()

const titleGroupAccordionValue = ref('0')
const editionGroupAccordionValue = ref('')
const torrentAccordionValue = ref('')
const uploadStep = ref(1)
const editionGroup = ref({})

const titleGroupDone = () => {
  titleGroupAccordionValue.value = ''
  editionGroupAccordionValue.value = '0'
  uploadStep.value = 2
}

const editionGroupDone = (editionGroup: object) => {
  editionGroup.value = editionGroup
  editionGroupStore.id = editionGroup.id
  editionGroupAccordionValue.value = ''
  torrentAccordionValue.value = '0'
  uploadStep.value = 3
}

const torrentDone = (torrent) => {
  router.push('/title-group/' + titleGroupStore.id + '?torrentId=' + torrent.id)
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
