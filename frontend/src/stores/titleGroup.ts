import { defineStore } from 'pinia'

export const useTitleGroupStore = defineStore('titleGroup', {
  state: () => {
    return {
      id: null,
      content_type: '',
      edition_groups: [],
    }
  },
})
