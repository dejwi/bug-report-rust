import { useSession } from 'next-auth/react'
import { useEffect } from 'react'

export default function Home() {
  const { status, data } = useSession()

  useEffect(() => {
    if (status === 'authenticated') {
      localStorage.setItem('token', data.user.token)
    } else if (status === 'unauthenticated') {
      localStorage.setItem('token', '')
    }
  }, [status, data])

  return <div>Blank</div>
}
