'use client'

import { yupResolver } from '@hookform/resolvers/yup'
import Link from 'next/link'
import { useForm } from 'react-hook-form'

import { AuthFields, authSchema } from './schema'

interface Props {
  onSubmit: (data: AuthFields) => void
  type: 'login' | 'register'
}
const AuthPage = ({ onSubmit, type }: Props) => {
  const { register, handleSubmit } = useForm<AuthFields>({
    resolver: yupResolver(authSchema),
    mode: 'onChange',
  })

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
          {type === 'login' ? 'Login' : 'Create Account'}
        </button>
        <span className="text-sm [&>a]:text-secondary">
          {type === 'login' ? (
            <>
              Don&apos;t have an account?{' '}
              <Link href="/register">Create one</Link>
            </>
          ) : (
            <>
              Already have an account? <Link href="/login">Login</Link>
            </>
          )}
        </span>
      </form>
    </div>
  )
}

export default AuthPage
