'use client'

import { useRouter } from 'next/navigation'

import AuthPage from '@/components/auth/page'

import { AuthFields } from '../../components/auth/schema'
import { useRegister } from '../../hooks/auth'

const RegisterPage = () => {
  const router = useRouter()

  const { mutate: register } = useRegister({
    onSuccess() {
      setTimeout(() => {
        router.push('/login')
      }, 600)
    },
  })

  const onSubmit = (data: AuthFields) => {
    register(data)
  }

  return <AuthPage onSubmit={onSubmit} type="register" />
}

export default RegisterPage
