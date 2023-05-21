import axios from 'axios'

export const api = axios.create({
  baseURL: process.env.BACKEND_URL,
})
api.interceptors.request.use(async request => {
  if (typeof window !== undefined) {
    const token = localStorage.gItem('token')
    if (token) {
      request.headers.common = {
        Authorization: `Bearer ${token}`,
      }
    }
  }

  return request
})
