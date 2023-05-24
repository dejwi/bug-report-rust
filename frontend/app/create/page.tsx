'use client'

import { useRouter } from 'next/navigation'

import BugReportForm from '@/components/bug-report/form'
import Navbar from '@/components/navbar'
import { useCreateBugReport } from '@/hooks/bug-reports'

const CreateBugReportPage = () => {
  const router = useRouter()
  const { mutate: create } = useCreateBugReport()

  return (
    <>
      <Navbar />
      <BugReportForm
        type="create"
        onSubmit={data => {
          create({
            data,
            onSuccess: id => setTimeout(() => router.push(`/${id}`), 700),
          })
        }}
      />
    </>
  )
}

export default CreateBugReportPage
