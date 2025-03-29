import api from './api.ts'

export const getTitleGroup = async (id: string | Number) => {
  try {
    return (await api.get('/title-group?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
export const getTitleGroupLite = async (id: string | Number) => {
  try {
    return (await api.get('/title-group/lite?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
export const createTitleGroup = async (titleGroup: object) => {
  try {
    const response = await api.post('/title-group', titleGroup)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
