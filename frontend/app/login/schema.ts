import * as yup from 'yup'

export const loginSchema = yup.object({
  username: yup.string().required(),
  password: yup.string().required(),
})

export type FormFields = yup.InferType<typeof loginSchema>
