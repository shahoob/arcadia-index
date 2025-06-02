import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Artist = components['schemas']['Artist']

export type ArtistLite = components['schemas']['ArtistLite']

export type AffiliatedArtistHierarchy = components['schemas']['AffiliatedArtistHierarchy']

export type ArtistAndTitleGroupsLite = components['schemas']['ArtistAndTitleGroupsLite']

export type TitleGroupHierarchyLite = components['schemas']['TitleGroupHierarchyLite']

export type UserCreatedAffiliatedArtist = components['schemas']['UserCreatedAffiliatedArtist']

export type UserCreatedArtist = components['schemas']['UserCreatedArtist']

export const getArtist = async (id: number): Promise<ArtistAndTitleGroupsLite> => {
  return (await api.get<ArtistAndTitleGroupsLite>('/artist?id=' + id)).data
}

export const createArtists = async (artists: UserCreatedArtist[]): Promise<Artist[]> => {
  return (await api.post<Artist[]>('/artists', artists)).data
}

export const searchArtistsLite = async (name: string): Promise<ArtistLite[]> => {
  return (await api.get<ArtistLite[]>(`/search/artist/lite?name=${name}`)).data
}
