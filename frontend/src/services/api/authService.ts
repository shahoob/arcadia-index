import api from './api.ts'

export const login = async (form: object) => {
  try {
    return (await api.post('/login', form)).data
  } catch (error) {
    console.error('API Error: ', error)
    throw error
  }
}

export const register = async (form: object) => {
  try {
    return (await api.post('/register', form)).data
  } catch (error) {
    console.error('API Error: ', error)
    throw error
  }
}
