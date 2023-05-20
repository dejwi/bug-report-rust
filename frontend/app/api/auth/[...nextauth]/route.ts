import NextAuth from 'next-auth/next'
import CredentialsProvider from 'next-auth/providers/credentials'

import { api } from '@/utils/api'

const handler = NextAuth({
  providers: [
    CredentialsProvider({
      // The name to display on the sign in form (e.g. "Sign in with...")
      name: 'Credentials',
      // `credentials` is used to generate a form on the sign in page.
      // You can specify which fields should be submitted, by adding keys to the `credentials` object.
      // e.g. domain, username, password, 2FA token, etc.
      // You can pass any HTML attribute to the <input> tag through the object.
      credentials: {
        username: {
          label: 'Username',
          type: 'text',
          placeholder: 'Bob',
        },
        password: { label: 'Password', type: 'password' },
      },
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      async authorize(credentials, req) {
        // Add logic here to look up the user from the credentials supplied
        const res = await api.post<{ token: string }>('/login', credentials)

        if (res.status !== 200) return null

        if (res.data.token) {
          // Any object returned will be saved in `user` property of the JWT
          return { id: res.data.token, token: res.data.token }
        }
        // If you return null then an error will be displayed advising the user to check their details.
        return null

        // You can also Reject this callback with an Error thus the user will be sent to the error page with the error message as a query parameter
      },
    }),
  ],
  callbacks: {
    async jwt({ token, user }) {
      return { ...token, ...user }
    },
    async session({ session, token }) {
      const sess = { ...session, user: token }
      return sess
    },
  },
  session: {
    strategy: 'jwt',
  },
  pages: {
    signIn: '/dolacz',
  },
})

export { handler as GET, handler as POST }
