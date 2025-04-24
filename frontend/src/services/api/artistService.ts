import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Artist = components['schemas']['Artist']

export type ArtistAndTitleGroupsLite = components['schemas']['ArtistAndTitleGroupsLite']

export type TitleGroupHierarchyLite = components['schemas']['TitleGroupHierarchyLite']

export const getArtist = async (id: number): Promise<ArtistAndTitleGroupsLite> => {
  try {
    return (await api.get<ArtistAndTitleGroupsLite>('/artist?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
