import './globals.css'

import clsx from 'clsx'
import { Inter } from 'next/font/google'
import { Toaster } from 'react-hot-toast'

import { NextAuthProvider } from '../components/wrappers/next-auth'
import ReactQueryWrapper from '../components/wrappers/react-query'
import TokenWrapper from '../components/wrappers/token'

const inter = Inter({ subsets: ['latin'] })

export const metadata = {
  title: 'BugReport app',
  description: 'App created as a frontend for a rust-learning backend project',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" data-theme="dracula" className="overflow-y-scroll">
      <ReactQueryWrapper>
        <NextAuthProvider>
          <TokenWrapper>
            <body
              className={clsx(inter.className, 'flex min-h-screen flex-col')}
            >
              {children}
              <Toaster />
            </body>
          </TokenWrapper>
        </NextAuthProvider>
      </ReactQueryWrapper>
    </html>
  )
}
