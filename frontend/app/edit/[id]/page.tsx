import { BugReportWithAuthor } from '@/types/types'
import { api } from '@/utils/api'

import EditBugReport from './edit'

const EditBugReportPage = async ({ params }: { params: { id: string } }) => {
  const { data } = await api.get<BugReportWithAuthor>(
    `/bug-reports/${params.id}`,
  )

  return <EditBugReport data={data} />
}

export default EditBugReportPage
