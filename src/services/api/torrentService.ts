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
    return (await api.post('/title-group', titleGroup)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
export const createEditionGroup = async (editionGroup: object) => {
  try {
    return (await api.post('/edition-group', editionGroup)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
