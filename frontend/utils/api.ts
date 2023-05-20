import axios from 'axios'

export const api = axios.create({
  baseURL: 'http://localhost:8000',
  headers: { Authorization: `Bearer ${localStorage.getItem('token')}` },
})
api.interceptors.request.use(async request => {
  if (typeof window !== undefined) {
    const token = localStorage.getItem('token')
    if (token) {
      request.headers.common = {
        Authorization: `Bearer ${token}`,
      }
    }
  }

  return request
})
