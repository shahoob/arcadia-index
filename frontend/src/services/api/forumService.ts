import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type ForumOverview = components['schemas']['ForumOverview']

export type ForumCategoryHierarchy = components['schemas']['ForumCategoryHierarchy']

export const getForum = async (): Promise<ForumOverview> => {
  try {
    return (await api.get<ForumOverview>('/forum')).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type ForumSubCategoryHierarchy = components['schemas']['ForumSubCategoryHierarchy']

export const getForumSubCategory = async (
  forumSubCategoryId: number,
): Promise<ForumSubCategoryHierarchy> => {
  try {
    return (
      await api.get<ForumSubCategoryHierarchy>('/forum/sub-category?id=' + forumSubCategoryId)
    ).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
