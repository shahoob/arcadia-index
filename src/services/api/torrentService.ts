import api from './api.ts';

export const getExampleData = async () => {
  try {
    const response = await api.get('/example-endpoint');
    return response.data;
  } catch (error) {
    console.error('API Error:', error);
    throw error;
  }
};
