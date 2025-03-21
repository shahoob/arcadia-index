<template>
  <div class="card">
    <ContentContainer>
      <div class="title">{{ title_group.name }}</div>

      <TitleGroupTable :edition_groups="edition_groups" />
    </ContentContainer>
  </div>
</template>

<script lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { getTitleGroup } from '@/services/api/torrentService'
import TitleGroupTable from '@/components/torrents/TitleGroupTable.vue'

export default {
  components: { ContentContainer, TitleGroupTable },
  setup() {},
  data() {
    return {
      title_group: {},
      edition_groups: [],
    }
  },
  created() {
    getTitleGroup(this.$route.query.id?.toString()).then((data) => {
      this.title_group = data.title_group
      this.edition_groups = data.edition_groups
    })
  },
}
</script>
