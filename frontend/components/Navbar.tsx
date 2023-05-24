'use client'

import Link from 'next/link'
import { signOut, useSession } from 'next-auth/react'

const Navbar = () => {
  const { status } = useSession()

  return (
    <div className="navbar">
      <div className="navbar-start gap-2">
        <Link className="btn-ghost btn text-xl normal-case" href="/">
          BugReport
        </Link>
      </div>
      <div className="navbar-center">
        <Link
          type="button"
          className="btn"
          href={status === 'authenticated' ? '/create' : '/login'}
        >
          CREATE
        </Link>
      </div>
      <div className="navbar-end">
        {status === 'authenticated' ? (
          <button className="btn" onClick={() => signOut()} type="button">
            LogOut
          </button>
        ) : (
          <Link href="/login" className="btn">
            Login
          </Link>
        )}
      </div>
    </div>
  )
}

export default Navbar
