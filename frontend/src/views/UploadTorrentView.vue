<template>
  <div>
    <div class="title">{{ t('torrent.upload_torrent') }}</div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="titleGroupDisabled">
        <AccordionHeader>
          <div style="display: flex">
            {{ t('title_group.title') }}
            <div v-if="titleGroupStore.id !== 0" style="display: flex">:<TitleGroupSlimHeader style="margin-left: 5px" :title_group="titleGroupStore" /></div>
          </div>
        </AccordionHeader>
        <AccordionContent>
          <!-- <CreateOrSelectTitleGroup @done="titleGroupDone" /> -->
          <CreateOrEditTitleGroup @done="titleGroupDone" :initial-title-group-form="null" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="editionGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>{{ t('torrent.edition') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrSelectEditionGroup @done="editionGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="torrentAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>{{ t('torrent.torrent') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrEditTorrent @done="torrentDone" />
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
import CreateOrSelectEditionGroup from '@/components/edition_group/CreateOrSelectEditionGroup.vue'
import CreateOrEditTorrent from '@/components/torrent/CreateOrEditTorrent.vue'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { EditionGroupInfoLite, TitleGroup, Torrent } from '@/services/api/torrentService'
import CreateOrEditTitleGroup from '@/components/title_group/CreateOrEditTitleGroup.vue'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
import { onMounted } from 'vue'

const router = useRouter()
const { t } = useI18n()

const editionGroupStore = useEditionGroupStore()
const titleGroupStore = ref(useTitleGroupStore())

const titleGroupAccordionValue = ref('0')
const titleGroupDisabled = ref(false)
const editionGroupAccordionValue = ref('0')
const torrentAccordionValue = ref('0')
const editionGroup = ref({})

const titleGroupDone = (titleGroup?: TitleGroup) => {
  titleGroupAccordionValue.value = ''
  titleGroupDisabled.value = true
  if (titleGroup) {
    titleGroupStore.value.id = titleGroup.id
    titleGroupStore.value.name = titleGroup.name
    console.log(titleGroup.name)
    titleGroupStore.value.original_release_date = titleGroup.original_release_date
  }
}

const editionGroupDone = (eg: EditionGroupInfoLite) => {
  editionGroup.value = eg
  editionGroupStore.id = eg.id
  // editionGroupAccordionValue.value = ''
  // torrentAccordionValue.value = '0'
}

const torrentDone = (torrent: Torrent) => {
  router.push('/title-group/' + titleGroupStore.value.id + '?torrentId=' + torrent.id)
}

onMounted(() => {
  if (titleGroupStore.value.id !== 0) {
    titleGroupDone()
  }
})
</script>

<style scoped>
.form {
  padding-top: 5px;
}
.title {
  margin-bottom: 20px;
  color: var(--color-primary);
}
.upload-step-accordion {
  margin-bottom: 20px;
}
</style>
