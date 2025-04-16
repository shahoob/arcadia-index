import { defineStore } from 'pinia'

interface SiteAppearanceSettings {
  item_detail_layout: string
}

interface UserSettings {
  site_appearance: SiteAppearanceSettings
}

interface User {
  avatar: string
  id: number
  settings: UserSettings
  username: string
}

export const useUserStore = defineStore('user', {
  state: () => ({
    avatar: '',
    id: 0,
    settings: {
      site_appearance: {
        item_detail_layout: '',
      },
    },
    username: '',
  }),
  actions: {
    setUser(user: User) {
      this.$state = { ...this.$state, ...user }
    },
  },
})
