import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedTitleGroupComment = components['schemas']['UserCreatedTitleGroupComment']

export type TitleGroupComment = components['schemas']['TitleGroupComment']

export type TitleGroupCommentHierarchy = components['schemas']['TitleGroupCommentHierarchy']

export const postTitleGroupComment = async (comment: UserCreatedTitleGroupComment) => {
  try {
    return (await api.post<TitleGroupComment>('/title-group-comment', comment)).data
  } catch (error) {
    throw error
  }
}
