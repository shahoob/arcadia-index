import type { TitleGroupLite } from '@/services/api/torrentService'
import { defineStore } from 'pinia'

export const useTitleGroupStore = defineStore('titleGroup', {
  state: (): TitleGroupLite => {
    return {
      id: 0,
      content_type: '',
      edition_groups: [],
      name: '',
    }
  },
})
