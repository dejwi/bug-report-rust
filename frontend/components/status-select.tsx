'use client'

import { useSession } from 'next-auth/react'
import { toast } from 'react-hot-toast'

import { BugReportStatus } from '@/types/types'

interface Props {
  authorId: string
  current: BugReportStatus

  onUpdateStatus: (status: BugReportStatus) => void
}

export const statusColors: Record<BugReportStatus, string> = {
  ACCEPTED: 'badge-success',
  CLOSED: 'badge-error',
  OPEN: 'badge-warning',
  REVIEW: 'badge-secondary',
  SOLVED: 'badge-info',
}

const BugReportStatusSelect = ({
  authorId,
  current,
  onUpdateStatus,
}: Props) => {
  const { status, data } = useSession()

  const onChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onUpdateStatus(e.target.value as BugReportStatus)
    e.target.blur()
  }

  const onClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation()
    if (status === 'unauthenticated') {
      toast.error('Please login in order to update report status')
    }
  }

  return (
    <button onClick={onClick} type="button">
      <select
        className={`badge flex cursor-pointer hover:brightness-90 ${statusColors[current]}`}
        onChange={onChange}
        style={{
          width: `${current.length + 2.7}ch`,
        }}
        disabled={status === 'unauthenticated'}
        value={current}
      >
        {Object.keys(statusColors).map(opt => (
          <option
            key={opt}
            selected={current === opt}
            disabled={
              current === opt ||
              (opt === 'SOLVED' && authorId !== data?.user.id)
            }
            value={opt}
          >
            {opt}
          </option>
        ))}
      </select>
    </button>
  )
}

export default BugReportStatusSelect
