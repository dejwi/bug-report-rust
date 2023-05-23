'use client'

import clsx from 'clsx'
import { AnimatePresence, motion } from 'framer-motion'
import { useRouter } from 'next/navigation'
import { useState } from 'react'
import { SyncLoader } from 'react-spinners'

import BugReport from '@/components/bug-report'
import { statusColors } from '@/components/status-select'
import { useBugReports } from '@/hooks/bug-reports'
import { BugReportStatus } from '@/types/types'

import Navbar from '../components/navbar'

export default function Home() {
  const router = useRouter()
  const [statusFilter, setStatusFilter] = useState<BugReportStatus[]>([])
  const { data, isLoading } = useBugReports({ status: statusFilter })

  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
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

      <AnimatePresence mode="popLayout">
        {isLoading ? (
          <motion.div
            key="loading-spinner"
            className="flex h-full flex-1 items-center justify-center"
            initial={{ y: -100, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: 100, opacity: 0 }}
          >
            <SyncLoader color="#837f97" speedMultiplier={0.7} />
          </motion.div>
        ) : (
          <motion.div
            key="reports"
            className="flex flex-col items-center gap-7"
            initial={{ y: -100, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: -100, opacity: 0 }}
          >
            {data?.map(rep => (
              <button
                onClick={() => {
                  router.push(`/${rep.id}`)
                }}
                type="button"
              >
                <BugReport key={rep.id} data={rep} truncateDescription />
              </button>
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  )
}
