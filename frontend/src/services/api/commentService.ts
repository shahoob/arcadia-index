import api from './api.ts'

export const postTitleGroupComment = async (comment: object) => {
  try {
    return (await api.post('/title-group-comment', comment)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
