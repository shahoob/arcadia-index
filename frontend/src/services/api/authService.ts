import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type Login = components['schemas']['Login']

export type LoginResponse = components['schemas']['LoginResponse']

export const login = async (form: Login): Promise<LoginResponse> => {
  try {
    return (await api.post<LoginResponse>('/login', form)).data
  } catch (error) {
    throw error
  }
}

export const register = async (form: object) => {
  try {
    return (await api.post('/register', form)).data
  } catch (error) {
    throw error
  }
}
