import React from 'react'
import clsx from 'clsx'

const variants = {
  primary: 'bg-green-500 text-white',
  inverse: 'bg-white text-gray-600 border',
  danger: 'bg-red-500 text-white'
}

const sizes = {
  sm: 'py-1 px-1 text-sm',
  md: 'py-1 px-2 text-md',
  lg: 'py-2 px-3 text-lg'
}

type BadgeProps = React.ComponentPropsWithoutRef<'span'> & {
  variant?: keyof typeof variants
  size?: keyof typeof sizes
  onClose?: () => void
}

export const Badge: React.FC<BadgeProps> = ({ variant = 'primary', size = 'md', onClose, className, ...props }) => {
  return (
    <span className={clsx('font-medium rounded inline-flex items-center', variants[variant], className)}>
      <span className={clsx(sizes[size])}>{props.children}</span>
      {onClose && (
        <span
          className={clsx('inline-flex items-center border-l h-full w-hull cursor-pointer', sizes[size])}
          onClick={() => onClose && onClose()}
        >
          x
        </span>
      )}
    </span>
  )
}