import { useMutation, useQuery } from '@tanstack/react-query'
import toast from 'react-hot-toast'

import { BugReportValues } from '@/components/bug-report/schema'
import { queryClient } from '@/components/wrappers/react-query'
import {
  BugReport,
  BugReportStatus,
  BugReportWithAuthor,
  UpdateBugReport,
} from '@/types/types'
import { api } from '@/utils/api'

type Filters = {
  status?: BugReportStatus[]
}

const parseFilters = (filters: Filters): string => {
  if (!filters.status?.length) return ''

  const params = new URLSearchParams()
  filters.status.forEach(s => params.append('status[]', s))

  return `?${params.toString()}`
}

export const useBugReports = (filters: Filters) =>
  useQuery<BugReportWithAuthor[]>({
    queryKey: ['bug-reports', 'all', { ...filters }],
    cacheTime: 0,
    queryFn: async () =>
      (await api.get(`/bug-reports${parseFilters(filters)}`)).data,
  })

export const useUpdateBugReport = (id: string, onSuccess?: () => void) =>
  useMutation({
    mutationFn: (body: UpdateBugReport) => {
      const statusToastId = toast.loading('Updating report', {
        duration: Infinity,
      })

      return new Promise((resolve, reject) => {
        api
          .put<BugReport>(`/bug-reports/${id}`, body)
          .then(() => {
            toast.success('Successfully updated report', {
              id: statusToastId,
              duration: 2000,
            })
            resolve(true)
          })
          .catch(err => {
            toast.error(err.response.data.message, {
              id: statusToastId,
              duration: 4000,
            })
            reject(err.response.data.message)
          })
      })
    },
    onSuccess: async (data, variables) => {
      const queryKey = ['bug-reports', 'all']
      await queryClient.cancelQueries({ queryKey })

      queryClient.setQueryData<BugReportWithAuthor[]>(queryKey, old => {
        const found = old?.find(r => r.id === id)
        if (found) {
          if (variables.status) found.status = variables.status
          if (variables.description) found.description = variables.description
          if (variables.title) found.title = variables.title
        }
        return old
      })

      queryClient.invalidateQueries({ queryKey })

      if (onSuccess) onSuccess()
    },
  })

export const useCreateBugReport = () =>
  useMutation({
    mutationFn: ({
      data,
      onSuccess,
    }: {
      data: BugReportValues
      onSuccess: (id: string) => void
    }) => {
      const statusToastId = toast.loading('Creating...', {
        duration: Infinity,
      })

      return new Promise((resolve, reject) => {
        api
          .post<BugReport>(`/bug-reports`, data)
          .then(res => {
            toast.success('Successfully created report', {
              id: statusToastId,
              duration: 2000,
            })
            onSuccess(res.data.id)
            resolve(true)
          })
          .catch(err => {
            toast.error(err.response.data.message, {
              id: statusToastId,
              duration: 4000,
            })
            reject(err.response.data.message)
          })
      })
    },
  })

export const useDeleteBugReport = (id: string, onSuccess?: () => void) =>
  useMutation({
    mutationFn: () => {
      const statusToastId = toast.loading('Deleting report', {
        duration: Infinity,
      })

      return new Promise((resolve, reject) => {
        api
          .delete<BugReport>(`/bug-reports/${id}`)
          .then(() => {
            toast.success('Successfully deleted report', {
              id: statusToastId,
              duration: 2000,
            })
            resolve(true)
          })
          .catch(err => {
            toast.error(err.response.data.message, {
              id: statusToastId,
              duration: 4000,
            })
            reject(err.response.data.message)
          })
      })
    },
    onSuccess: async () => {
      const queryKey = ['bug-reports', 'all']
      await queryClient.cancelQueries({ queryKey })

      queryClient.setQueryData<BugReportWithAuthor[]>(queryKey, old =>
        old?.filter(r => r.id !== id),
      )

      queryClient.invalidateQueries({ queryKey })

      if (onSuccess) onSuccess()
    },
  })
