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
          <CreateOrSelectTitleGroup @done="titleGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion
      :value="editionGroupAccordionValue"
      class="upload-step-accordion"
      v-tooltip.top="{
        value: t('edition_group.complete_title_group_first'),
        disabled: titleGroupDisabled,
      }"
    >
      <AccordionPanel value="0" :disabled="editionGroupDisabled || !titleGroupDisabled">
        <AccordionHeader>
          <span>
            {{ t('torrent.edition') }}<span v-if="editionGroup" class="bold">: {{ getEditionGroupSlug(editionGroup) }}</span>
          </span>
        </AccordionHeader>
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
import type { EditionGroupInfoLite, TitleGroup, TitleGroupLite, Torrent } from '@/services/api/torrentService'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
import { onMounted } from 'vue'
import { getEditionGroupSlug } from '@/services/helpers'
import CreateOrSelectTitleGroup from '@/components/title_group/CreateOrSelectTitleGroup.vue'

const router = useRouter()
const { t } = useI18n()

const editionGroupStore = ref(useEditionGroupStore())
const titleGroupStore = ref(useTitleGroupStore())

const titleGroupAccordionValue = ref('0')
const titleGroupDisabled = ref(false)
const editionGroupAccordionValue = ref('0')
const editionGroupDisabled = ref(false)
const torrentAccordionValue = ref('0')
const editionGroup = ref<EditionGroupInfoLite | null>(null)
const siteName = import.meta.env.VITE_SITE_NAME

const titleGroupDone = (titleGroup?: TitleGroup | TitleGroupLite) => {
  titleGroupAccordionValue.value = ''
  titleGroupDisabled.value = true
  if (titleGroup) {
    titleGroupStore.value.id = titleGroup.id
    titleGroupStore.value.content_type = titleGroup.content_type
    titleGroupStore.value.name = titleGroup.name
    titleGroupStore.value.original_release_date = titleGroup.original_release_date
    if ('edition_groups' in titleGroup) {
      titleGroupStore.value.edition_groups = titleGroup.edition_groups
    }
  }
}

const editionGroupDone = (eg: EditionGroupInfoLite) => {
  editionGroup.value = eg
  editionGroupStore.value.id = eg.id
  editionGroupAccordionValue.value = ''
  editionGroupDisabled.value = true
  // torrentAccordionValue.value = '0'
}

const torrentDone = (torrent: Torrent) => {
  router.push('/title-group/' + titleGroupStore.value.id + '?torrentId=' + torrent.id)
}

onMounted(() => {
  document.title = `Upload torrent - ${siteName}`

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
