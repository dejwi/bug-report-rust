import axios from 'axios'

export const api = axios.create({
  baseURL: process.env.BACKEND_URL,
})
api.interceptors.request.use(async request => {
  const token =
    typeof window !== 'undefined' ? window.localStorage.getItem('token') : ''

  if (token) {
    request.headers.Authorization = `Bearer ${token}`
  }

  return request
})
