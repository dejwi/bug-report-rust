import BugReport from '@/components/bug-report/bug-report'
import Navbar from '@/components/navbar'
import { BugReportWithAuthor } from '@/types/types'
import { api } from '@/utils/api'

const SingleReportPage = async ({ params }: { params: { id: string } }) => {
  const { data } = await api.get<BugReportWithAuthor>(
    `/bug-reports/${params.id}`,
  )

  return (
    <div className=" flex min-h-screen flex-col">
      <Navbar />
      <div className="flex flex-1 items-center justify-center">
        <BugReport data={data} />
      </div>
    </div>
  )
}

export default SingleReportPage
