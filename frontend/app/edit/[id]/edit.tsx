'use client'

import { useRouter } from 'next/navigation'

import BugReportForm from '@/components/bug-report/form'
import Navbar from '@/components/navbar'
import { useUpdateBugReport } from '@/hooks/bug-reports'
import { BugReportWithAuthor } from '@/types/types'

interface Props {
  data: BugReportWithAuthor
}

const EditBugReport = ({ data }: Props) => {
  const router = useRouter()
  const { mutate: update } = useUpdateBugReport(data.id, () => {
    setTimeout(() => router.push(`/${data.id}`), 2000)
  })

  return (
    <>
      <Navbar />
      <BugReportForm
        type="edit"
        description={data.description}
        title={data.title}
        onSubmit={updateData => {
          update(updateData)
        }}
      />
    </>
  )
}

export default EditBugReport
