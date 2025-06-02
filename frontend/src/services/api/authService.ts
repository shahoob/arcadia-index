import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type Login = components['schemas']['Login']

export type LoginResponse = components['schemas']['LoginResponse']

export const login = async (form: Login): Promise<LoginResponse> => {
  return (await api.post<LoginResponse>('/login', form)).data
}

export const register = async (form: object) => {
  return (await api.post('/register', form)).data
}
