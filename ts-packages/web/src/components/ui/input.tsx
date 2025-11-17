import * as React from 'react';

import { cn } from '@/lib/utils';

export interface InputProps extends React.ComponentProps<'input'> {
  /**
   * Visual variant of the input
   * - 'default': Standard input styling
   * - 'post': Uses post creation theme colors
   */
  variant?: 'default' | 'post';
  /**
   * Show a red asterisk to indicate required field
   */
  showRequiredMarker?: boolean;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      type,
      variant = 'default',
      showRequiredMarker = false,
      placeholder,
      ...props
    },
    ref,
  ) => {
    const variantClasses = {
      default:
        'border-input-box-border bg-input-box-bg dark:bg-input/30 placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50',
      post: '!bg-post-input-bg !border-post-input-border placeholder:text-[var(--color-post-input-placeholder)] focus-visible:border-post-input-border focus-visible:ring-post-input-border/30',
    };

    // Calculate approximate position based on placeholder length
    const getMarkerPosition = () => {
      if (!placeholder || !showRequiredMarker) return '2.7rem';
      // Approximate: 0.8rem base + 0.45rem per character
      const baseOffset = 0.8;
      const charWidth = 0.45;
      return `${baseOffset + placeholder.length * charWidth}rem`;
    };

    return (
      <div className="relative w-full">
        <input
          type={type}
          ref={ref}
          data-slot="input"
          placeholder={placeholder}
          className={cn(
            'flex h-9 w-full min-w-0 rounded-md px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none',
            'text-text-primary file:text-text-primary',
            'file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium',
            'selection:bg-primary selection:text-primary-foreground',
            'border',
            variantClasses[variant],
            'focus-visible:ring-[1px]',
            'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive aria-invalid:outline aria-invalid:border-c-p-50',
            'disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50',
            'md:text-sm',
            className,
          )}
          {...props}
        />
        {showRequiredMarker && (
          <span
            className="pointer-events-none absolute top-1/2 -translate-y-1/2 text-post-required-marker"
            style={{ left: getMarkerPosition() }}
            aria-hidden="true"
          >
            *
          </span>
        )}
      </div>
    );
  },
);

Input.displayName = 'Input';

export { Input };
