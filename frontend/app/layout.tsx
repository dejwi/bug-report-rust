import './globals.css'

import { Inter } from 'next/font/google'

import ReactQueryWrapper from './react-query-wrapper'

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
    <html lang="en">
      <ReactQueryWrapper>
        <body className={inter.className}>{children}</body>
      </ReactQueryWrapper>
    </html>
  )
}
