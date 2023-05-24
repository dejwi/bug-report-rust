import * as yup from 'yup'

export const bugReportSchema = yup.object({
  title: yup.string().required(),
  description: yup.string().notRequired(),
})

export interface BugReportValues {
  title: string
  description?: string
}
