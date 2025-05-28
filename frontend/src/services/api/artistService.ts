import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Artist = components['schemas']['Artist']

export type AffiliatedArtistHierarchy = components['schemas']['AffiliatedArtistHierarchy']

export type ArtistAndTitleGroupsLite = components['schemas']['ArtistAndTitleGroupsLite']

export type TitleGroupHierarchyLite = components['schemas']['TitleGroupHierarchyLite']

export type UserCreatedAffiliatedArtist = components['schemas']['UserCreatedAffiliatedArtist']

export type UserCreatedArtist = components['schemas']['UserCreatedArtist']

export const getArtist = async (id: number): Promise<ArtistAndTitleGroupsLite> => {
  try {
    return (await api.get<ArtistAndTitleGroupsLite>('/artist?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export const createArtists = async (artists: UserCreatedArtist[]): Promise<Artist[]> => {
  try {
    return (await api.post<Artist[]>('/artists', artists)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
