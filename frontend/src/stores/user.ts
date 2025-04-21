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
  uploaded: number
  downloaded: number
  ratio: number
  bonus_points: number
  freeleech_tokens: number
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
    uploaded: 0,
    downloaded: 0,
    ratio: 0.0,
    bonus_points: 0,
    freeleech_tokens: 0,
  }),
  actions: {
    setUser(user: User) {
      this.$state = { ...this.$state, ...user }
    },
  },
})
