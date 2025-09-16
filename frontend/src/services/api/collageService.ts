import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type CollageAndAssociatedData = components['schemas']['CollageAndAssociatedData']

export type Collage = components['schemas']['Collage']

export type CollageSearchResponse = components['schemas']['CollageSearchResponse']

export type SearchCollagesQuery = components['schemas']['SearchCollagesQuery']

export type CollageSearchResult = components['schemas']['CollageSearchResult']

export type UserCreatedCollage = components['schemas']['UserCreatedCollage']

export type CollageType = components['schemas']['CollageType']

export type CollageCategory = components['schemas']['CollageCategory']

export type UserCreatedCollageEntry = components['schemas']['UserCreatedCollageEntry']

export type CollageEntry = components['schemas']['CollageEntry']

export const getCollage = async (id: number): Promise<CollageAndAssociatedData> => {
  return (await api.get<CollageAndAssociatedData>('/collages?id=' + id)).data
}

export const searchCollages = async (form: SearchCollagesQuery): Promise<CollageSearchResponse> => {
  return (await api.get<CollageSearchResponse>('/search/collages', { params: form })).data
}

export const createCollage = async (form: UserCreatedCollage): Promise<Collage> => {
  return (await api.post<Collage>('/collages', form)).data
}

export const createCollageEntries = async (form: UserCreatedCollageEntry[]): Promise<CollageEntry[]> => {
  return (await api.post<CollageEntry[]>('/collages/entries', form)).data
}
