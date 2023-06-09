'use client'

import { useRouter } from 'next/navigation'

import AuthPage from '@/components/auth/page'

import { AuthFields } from '../../components/auth/schema'
import { useLogin } from '../../hooks/auth'

const LoginPage = () => {
  const router = useRouter()

  const { mutate: login } = useLogin({
    onSuccess() {
      setTimeout(() => {
        router.replace('/')
      }, 600)
    },
  })

  const onSubmit = (data: AuthFields) => {
    login(data)
  }

  return <AuthPage onSubmit={onSubmit} type="login" />
}

export default LoginPage
