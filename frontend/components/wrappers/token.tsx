'use client'

import { useSession } from 'next-auth/react'
import { useEffect } from 'react'

const TokenWrapper = ({ children }: { children: React.ReactNode }) => {
  const { status, data } = useSession()

  useEffect(() => {
    if (typeof window !== undefined) {
      if (status === 'authenticated') {
        localStorage.setItem('token', data.user.accessToken)
      } else if (status === 'unauthenticated') {
        localStorage.setItem('token', '')
      }
    }
  }, [status, data])

  return children as JSX.Element
}

export default TokenWrapper
