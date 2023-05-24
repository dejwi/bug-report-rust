import axios from 'axios'
import { NextAuthOptions } from 'next-auth'
import NextAuth from 'next-auth/next'
import CredentialsProvider from 'next-auth/providers/credentials'

const options: NextAuthOptions = {
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
        return axios
          .post<{ token?: string; message?: string; id?: string }>(
            `${process.env.BACKEND_URL}/login`,
            credentials,
          )
          .then(res => {
            if (res.data?.token && res.data?.id) {
              return { id: res.data.id, accessToken: res.data.token }
            }
            throw new Error(res.data.message)
          })
          .catch(err => {
            throw new Error(err.response.data.message)
          })
      },
    }),
  ],
  callbacks: {
    jwt: async ({ token, user }) => {
      const out = {} as { id: string; accessToken: string }
      if (user) {
        out.accessToken = user.accessToken
        out.id = user.id
      }
      return { ...token, ...out }
    },
    session: ({ session, token }) => ({
      ...session,
      user: {
        accessToken: token?.accessToken as string,
        id: token?.id as string,
      },
    }),
  },
  session: {
    strategy: 'jwt',
  },
  pages: {
    signIn: '/login',
    signOut: '/',
  },
}

export default NextAuth(options)
