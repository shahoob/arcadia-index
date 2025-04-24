import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedTitleGroupComment = components['schemas']['Artist']

export const postTitleGroupComment = async (comment: UserCreatedTitleGroupComment) => {
  try {
    return (await api.post('/title-group-comment', comment)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
