import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type HomePage = components['schemas']['HomePage']

export type ForumPostAndThreadName = components['schemas']['ForumPostAndThreadName']

export type HomeStats = components['schemas']['HomeStats']

export const getHome = async () => {
  return (await api.get<HomePage>('/home')).data
}
