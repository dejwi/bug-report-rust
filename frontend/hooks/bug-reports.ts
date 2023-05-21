import { useQuery } from '@tanstack/react-query'

import { BugReportWithAuthor } from '@/types/types'
import { api } from '@/utils/api'

export const useBugReports = () =>
  useQuery<BugReportWithAuthor[]>({
    queryKey: ['bug-reports', 'all'],
    queryFn: () => api.get('/bug-reports'),
  })
