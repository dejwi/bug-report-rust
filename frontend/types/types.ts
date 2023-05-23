export type BugReportStatus =
  | 'OPEN'
  | 'CLOSED'
  | 'SOLVED'
  | 'REVIEW'
  | 'ACCEPTED'

export interface BugReport {
  id: string
  title: string
  description?: string
  status: BugReportStatus
  authorId: string
  createdAt: string
}

export interface BugReportWithAuthor extends Omit<BugReport, 'authorId'> {
  author: {
    id: string
    username: string
  }
}
export interface UpdateBugReport {
  title?: string
  description?: string
  status?: BugReportStatus
}
