'use client'

import { useUpdateBugReport } from '@/hooks/bug-reports'
import { BugReportStatus, BugReportWithAuthor } from '@/types/types'

import BugReportStatusSelect from './status-select'

const txt = `Lorem ipsum dolor sit amet consectetur adipisicing elit. Est repellat incidunt quas deserunt distinctio! Nisi sit quaerat iure.

      Error inventore molestias illo possimus nam beatae tempore maxime animi facere nemo veritatis, voluptatum facilis corrupti molestiae praesentium deleniti iure.

      Aliquam, explicabo odio. Accusantium corporis aspernatur quibusdam! Magnam recusandae iusto ullam, laudantium esse illo, deserunt voluptatibus est expedita nobis aliquid voluptate repellat.`

const bigtxt = txt + txt + txt
interface Props {
  data: BugReportWithAuthor
  truncateDescription?: boolean
}
const BugReport = ({
  data: { author, status, title, description, id },
  truncateDescription,
}: Props) => {
  const { mutate: updateReport } = useUpdateBugReport(id)

  let text = bigtxt
  if (truncateDescription && text.length > 500) {
    text = `${text.slice(0, 500)}...`
  }

  const onUpdateStatus = (val: BugReportStatus) => {
    updateReport({ status: val })
  }

  return (
    <div
      className="flex w-[30rem] max-w-[calc(100%-1rem)] cursor-pointer flex-col px-5 py-7 shadow-xl transition-transform hover:-translate-x-1 hover:-translate-y-1"
      role="button"
    >
      <div className="flex gap-1">
        <BugReportStatusSelect
          current={status}
          authorId={author.id}
          onUpdateStatus={onUpdateStatus}
        />
        <span className="self-end text-xs">by {author.username}</span>
      </div>

      <h2 className="mt-1 text-center text-2xl">{title}</h2>

      <p className="whitespace-pre-wrap text-center text-sm text-base-content/80">
        {text}
      </p>
    </div>
  )
}

export default BugReport
