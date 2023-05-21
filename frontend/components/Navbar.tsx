'use client'

import Link from 'next/link'
import { signOut, useSession } from 'next-auth/react'

const Navbar = () => {
  const { status } = useSession()

  return (
    <div className="navbar bg-base-100">
      <div className="navbar-start gap-2">
        <Link className="btn-ghost btn text-xl normal-case" href="/">
          BugReport
        </Link>
        {status === 'authenticated' && (
          <button type="button" className="btn-secondary btn">
            ADD
          </button>
        )}
      </div>
      <div className="navbar-end">
        {status === 'authenticated' ? (
          <button className="btn" onClick={() => signOut()} type="button">
            LogOut
          </button>
        ) : (
          <Link href="/login" className="btn-primary btn">
            Login
          </Link>
        )}
      </div>
    </div>
  )
}

export default Navbar
