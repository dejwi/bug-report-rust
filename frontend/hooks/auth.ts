'use client'

import { useMutation } from '@tanstack/react-query'
import { signIn } from 'next-auth/react'
import toast from 'react-hot-toast'

import { api } from '@/utils/api'

import { AuthFields } from '../components/auth/schema'

interface Options {
  onSuccess?: () => void
  onError?: () => void
}

export const useLogin = (opts?: Options) =>
  useMutation({
    ...opts,
    mutationFn: ({ password, username }: AuthFields) => {
      const statusToastId = toast.loading('Checking your credentials', {
        duration: Infinity,
      })

      return new Promise((resolve, reject) => {
        signIn('credentials', {
          redirect: false,
          username,
          password,
        }).then(res => {
          if (res?.ok) {
            toast.success('Successfully logged in', {
              id: statusToastId,
              duration: 2000,
            })
            resolve(true)
          } else {
            toast.error(res?.error || 'Invalid credentails', {
              id: statusToastId,
              duration: 4000,
            })
            reject(res?.error)
          }
        })
      })
    },
  })

export const useRegister = (opts?: Options) =>
  useMutation({
    ...opts,
    mutationFn: (body: AuthFields) => {
      const statusToastId = toast.loading('Creating account...', {
        duration: Infinity,
      })

      return new Promise((resolve, reject) => {
        api
          .post('/register', body)
          .then(() => {
            toast.success('Successfully created account', {
              id: statusToastId,
              duration: 2000,
            })
            resolve(true)
          })
          .catch(err => {
            toast.error(err.response.data.message, {
              id: statusToastId,
              duration: 4000,
            })
            reject(err.response.data.message)
          })
      })
    },
  })
