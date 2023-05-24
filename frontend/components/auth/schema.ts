import * as yup from 'yup'

export const authSchema = yup.object({
  username: yup.string().required(),
  password: yup.string().required(),
})

export type AuthFields = yup.InferType<typeof authSchema>
