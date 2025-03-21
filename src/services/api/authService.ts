import api from './api.ts';

export const login = async (form: Object) => {
  try {
    const response = await api.post('/login', form);
    return response.data;
  } catch (error) {
    console.error('API Error: ', error);
    throw error;
  }
};

export const register = async (form: Object) => {
  try {
    const response = await api.post('/register', form);
    return response.data;
  } catch (error) {
    console.error('API Error: ', error);
    throw error;
  }
};