// eslint-disable-next-line @typescript-eslint/no-unused-vars
import NextAuth from 'next-auth'

declare module 'next-auth' {
  interface User {
    accessToken: string
    id: string
  }

  interface Session {
    user: {
      accessToken: string
      id: string
    }
  }
}
