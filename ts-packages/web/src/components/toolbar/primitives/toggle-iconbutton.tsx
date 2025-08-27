import { cn } from '@/lib/utils';
import * as React from 'react';

interface ToggleIconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  active?: boolean;
  size?: number;
}

export const ToggleIconButton = React.forwardRef<HTMLButtonElement, ToggleIconButtonProps>(
  ({ active, className, children, size = 24, ...rest }, ref) => {
    return (
      <button
        ref={ref}
        {...rest}
        className={cn(
          'rounded hover:bg-neutral-700 inline-flex items-center justify-center',
          active && 'bg-neutral-600',
          className,
        )}
        style={{ width: size, height: size }}
      >
        {children}
      </button>
    );
  },
);
ToggleIconButton.displayName = 'ToggleIconButton';
