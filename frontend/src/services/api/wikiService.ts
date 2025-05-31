import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type WikiArticle = components['schemas']['WikiArticle']

export const getWikiArticle = async (articleId: number) => {
  try {
    return (await api.get<WikiArticle>(`/wiki/article?id=${articleId}`)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
