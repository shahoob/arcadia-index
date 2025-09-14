import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type CollageAndAssociatedData = components['schemas']['CollageAndAssociatedData']

export type Collage = components['schemas']['Collage']

export const getCollage = async (id: number): Promise<CollageAndAssociatedData> => {
  return (await api.get<CollageAndAssociatedData>('/collages?id=' + id)).data
}
