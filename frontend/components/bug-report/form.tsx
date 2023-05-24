'use client'

import { yupResolver } from '@hookform/resolvers/yup'
import clsx from 'clsx'
import { useForm } from 'react-hook-form'

import { bugReportSchema, BugReportValues } from './schema'

interface Props extends Partial<BugReportValues> {
  onSubmit: (data: BugReportValues) => void
  type: 'edit' | 'create'
}

const BugReportForm = ({ description, title, onSubmit, type }: Props) => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    mode: 'onChange',
    resolver: yupResolver(bugReportSchema),
    defaultValues: {
      description,
      title,
    },
  })

  return (
    <div className="flex flex-1 items-center justify-center">
      <form
        onSubmit={handleSubmit(
          onSubmit as (data: Partial<BugReportValues>) => void,
        )}
        className="flex w-[30rem] max-w-[calc(100%-2rem)] flex-col gap-2"
      >
        <input
          type="text"
          className={clsx(
            'input-bordered input',
            errors.title?.message && 'input-error',
          )}
          placeholder="Title"
          {...register('title')}
        />
        <textarea
          className="textarea-bordered textarea textarea-md h-[13rem]"
          placeholder="Description"
          {...register('description')}
        />
        <button type="submit" className="btn">
          {type === 'edit' ? 'SAVE' : 'CREATE'}
        </button>
      </form>
    </div>
  )
}

export default BugReportForm
