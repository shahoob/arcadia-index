<template>
  <ContentContainer>
    <div id="torrent-search-inputs">
      <FloatLabel>
        <InputText
          class="title-group-name"
          size="small"
          v-model="searchForm.title_group_name"
          name="title_group_name"
        />
        <label for="title_group_name">Search terms</label>
      </FloatLabel>
      <FloatLabel>
        <InputText class="tags" size="small" v-model="searchForm.tags" name="tags" />
        <label for="tags">Tags (comma separated)</label>
      </FloatLabel>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading label="Search" @click="$emit('search', searchForm)" />
      </div>
    </div>
  </ContentContainer>
</template>

<script lang="ts">
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Button from 'primevue/button'
export default {
  components: { ContentContainer, Button, InputText, FloatLabel },
  props: { loading: { default: false }, initialTitleGroupName: {} },
  data() {
    return {
      searchForm: {
        title_group_name: '',
      },
    }
  },
  created() {
    this.searchForm.title_group_name = this.initialTitleGroupName
  },
}
</script>

<style>
#torrent-search-inputs {
  padding-top: 20px;
}
.title-group-name {
  width: 40%;
}
.tags {
  width: 30%;
}
.p-floatlabel {
  margin-bottom: 25px;
}
</style>
