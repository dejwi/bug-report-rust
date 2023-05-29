'use client'

import Link from 'next/link'
import { useSession } from 'next-auth/react'
import { useState } from 'react'

import { useDeleteBugReport, useUpdateBugReport } from '@/hooks/bug-reports'
import { BugReportStatus, BugReportWithAuthor } from '@/types/types'

import Modal from '../modal'
import DeleteIcon from './delete.svg'
import EditIcon from './edit.svg'
import BugReportStatusSelect from './status-select'

interface Props {
  data: BugReportWithAuthor
  truncateDescription?: boolean
  asLink?: boolean
}
const BugReport = ({
  data: { author, status, title, description, id },
  truncateDescription,
  asLink,
}: Props) => {
  const { data: session } = useSession()
  const { mutate: updateReport } = useUpdateBugReport(id)
  const [showConfirmDelete, setShowConfirmDelete] = useState(false)
  const { mutate: deleteReport } = useDeleteBugReport(id)

  let text = description || ''
  if (truncateDescription && text.length > 500) {
    text = `${text.slice(0, 500)}...`
  }

  const onUpdateStatus = (val: BugReportStatus) => {
    updateReport({ status: val })
  }

  return (
    <>
      <div
        className="flex w-[30rem] max-w-[calc(100%-1rem)] cursor-pointer flex-col px-5 py-7 shadow-xl transition-transform hover:-translate-x-1 hover:-translate-y-1"
        role="button"
      >
        <div className="flex items-center justify-between">
          <div className="flex gap-1">
            <BugReportStatusSelect
              current={status}
              authorId={author.id}
              onUpdateStatus={onUpdateStatus}
            />
            <span className="self-end text-xs">by {author.username}</span>
          </div>
          {session?.user.id === author.id && (
            <div className="flex items-center justify-center">
              <Link
                type="button"
                href={`/edit/${id}`}
                className="btn-ghost btn h-auto min-h-[auto] w-auto p-1"
              >
                <EditIcon className="h-5 w-5 fill-base-content" />
              </Link>
              <button
                type="button"
                className="btn-ghost btn h-auto min-h-[auto] w-auto p-1"
                onClick={() => setShowConfirmDelete(true)}
              >
                <DeleteIcon className="h-5 w-5 fill-base-content" />
              </button>
            </div>
          )}
        </div>

        <OptionalLink href={asLink ? `/${id}` : undefined}>
          <h2 className="mt-1 text-center text-2xl">{title}</h2>

          <p className="whitespace-pre-wrap text-center text-sm text-base-content/80">
            {text}
          </p>
        </OptionalLink>
      </div>
      <Modal open={showConfirmDelete}>
        <button
          className="btn-sm btn-circle btn absolute right-2 top-2"
          type="button"
          onClick={() => setShowConfirmDelete(false)}
        >
          ✕
        </button>
        <h3 className="text-lg font-bold">{title}</h3>
        <p className="py-4">Are you sure you want to delete this bug report?</p>
        <div className="modal-action">
          <button
            className="btn"
            type="button"
            onClick={() => {
              setShowConfirmDelete(false)
              deleteReport()
            }}
          >
            Yes, delete
          </button>
        </div>
      </Modal>
    </>
  )
}

export default BugReport

interface WrapperProps {
  href?: string
  children: React.ReactNode
  className?: string
}

const OptionalLink = ({ href, children, className }: WrapperProps) =>
  href ? (
    <Link href={href} className={className}>
      {children}
    </Link>
  ) : (
    <div className={className}>{children}</div>
  )
