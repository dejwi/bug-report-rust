import clsx from 'clsx'

interface Props {
  children: React.ReactNode
  open: boolean
}

const Modal = ({ children, open }: Props) => (
  <div
    className={clsx('modal modal-bottom sm:modal-middle', open && 'modal-open')}
  >
    <div className="modal-box relative">{children}</div>
  </div>
)

export default Modal
