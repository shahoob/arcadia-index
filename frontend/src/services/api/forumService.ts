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

export type ForumThreadsAndPosts = components['schemas']['ForumThreadAndPosts']

export type ForumPostHierarchy = components['schemas']['ForumPostHierarchy']

export const getForumThread = async (forumThreadId: number): Promise<ForumThreadsAndPosts> => {
  try {
    return (await api.get<ForumThreadsAndPosts>('/forum/thread?id=' + forumThreadId)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type UserCreatedForumPost = components['schemas']['UserCreatedForumPost']

export type ForumPost = components['schemas']['ForumPost']

export const postForumPost = async (form: UserCreatedForumPost): Promise<ForumPost> => {
  try {
    return (await api.post<ForumPost>('/forum/post', form)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
