'use client'

import { yupResolver } from '@hookform/resolvers/yup'
import { useRouter } from 'next/navigation'
import { useForm } from 'react-hook-form'

import { useLogin } from '../../hooks/auth'
import { FormFields, loginSchema } from './schema'

const LoginPage = () => {
  const router = useRouter()
  const { register, handleSubmit } = useForm<FormFields>({
    resolver: yupResolver(loginSchema),
    mode: 'onChange',
  })

  const { mutate: login } = useLogin({
    onSuccess() {
      setTimeout(() => {
        router.push('/')
      }, 600)
    },
  })

  const onSubmit = (data: FormFields) => {
    login(data)
  }

  return (
    <div className="flex h-screen w-full items-center justify-center">
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
        <input
          type="text"
          placeholder="Username"
          className="input-bordered input w-full max-w-xs"
          {...register('username')}
        />
        <input
          type="password"
          placeholder="Password"
          className="input-bordered input w-full max-w-xs"
          {...register('password')}
        />
        <button type="submit" className="btn mt-3">
          Login
        </button>
      </form>
    </div>
  )
}

export default LoginPage
