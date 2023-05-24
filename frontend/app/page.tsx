'use client'

import clsx from 'clsx'
import { AnimatePresence, motion } from 'framer-motion'
import { useState } from 'react'

import BugReport from '@/components/bug-report/bug-report'
import { statusColors } from '@/components/bug-report/status-select'
import { useBugReports } from '@/hooks/bug-reports'
import { BugReportStatus } from '@/types/types'

import Navbar from '../components/navbar'

export default function Home() {
  const [statusFilter, setStatusFilter] = useState<BugReportStatus[]>([])
  const { data } = useBugReports({ status: statusFilter })

  return (
    <>
      <Navbar />
      <div className="flex-1">
        <div className="sticky top-0 z-50 border-t-2 border-base-content/10 bg-base-100 pt-5">
          <div className="flex flex-wrap items-center justify-center gap-2 ">
            {Object.keys(statusColors).map(opt => (
              <button
                key={`filter-${opt}`}
                type="button"
                className={clsx(
                  'badge transition',
                  statusColors[opt as BugReportStatus],
                  !statusFilter.includes(opt as BugReportStatus) &&
                    '[&:not(:hover)]:badge-outline',
                )}
                onClick={() => {
                  if (!statusFilter.includes(opt as BugReportStatus))
                    setStatusFilter(prev => [...prev, opt as BugReportStatus])
                  else setStatusFilter(prev => prev.filter(f => f !== opt))
                }}
              >
                {opt}
              </button>
            ))}
          </div>
          <div className="divider" />
        </div>

        <AnimatePresence mode="wait">
          {data && (
            <motion.div
              key="reports"
              className="flex flex-col items-center gap-7"
              initial={{ y: -70, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              exit={{ y: -100, opacity: 0 }}
            >
              {data?.map(rep => (
                <BugReport key={rep.id} data={rep} truncateDescription asLink />
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </>
  )
}
