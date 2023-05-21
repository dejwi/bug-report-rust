'use client'

import { useMutation } from '@tanstack/react-query'
import { signIn } from 'next-auth/react'
import toast from 'react-hot-toast'

import { FormFields } from '../app/login/schema'

interface Options {
  onSuccess?: () => void
  onError?: () => void
}

export const useLogin = (opts?: Options) =>
  useMutation({
    ...opts,
    mutationFn: ({ password, username }: FormFields) => {
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
