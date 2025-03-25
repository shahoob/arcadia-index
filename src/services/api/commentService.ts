import api from './api.ts'

export const postTitleGroupComment = async (comment: object) => {
  try {
    const response = await api.post('/title-group-comment', comment)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
