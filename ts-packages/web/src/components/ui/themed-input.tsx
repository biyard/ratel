import * as React from 'react';

import { cn } from '@/lib/utils';

export interface ThemedInputProps extends React.ComponentProps<'input'> {
  /**
   * Visual variant of the input
   * - 'default': Standard input styling
   * - 'post': Uses post creation theme colors (#101010 dark, #a3a3a3 light)
   */
  variant?: 'default' | 'post';
  /**
   * Show a red asterisk to indicate required field
   */
  showRequiredMarker?: boolean;
}

/**
 * Themed input component with seamless light/dark mode support
 * Uses theme color variables for consistent appearance
 */
const ThemedInput = React.forwardRef<HTMLInputElement, ThemedInputProps>(
  (
    {
      className,
      type,
      variant = 'default',
      showRequiredMarker = false,
      ...props
    },
    ref,
  ) => {
    const variantClasses = {
      default:
        'border-input-box-border bg-input-box-bg focus-visible:border-ring focus-visible:ring-ring/50',
      post: '!bg-post-input-bg !border-post-input-border focus-visible:border-post-input-border focus-visible:ring-post-input-border/30',
    };

    return (
      <div className="relative w-full">
        <input
          type={type}
          ref={ref}
          data-slot="input"
          className={cn(
            // Base styles
            'flex h-9 w-full min-w-0 rounded-md px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none',
            'text-text-primary placeholder:text-gray-600',
            'file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-text-primary',
            'selection:bg-primary selection:text-primary-foreground',
            // Border and background
            'border',
            variantClasses[variant],
            // Focus states
            'focus-visible:ring-[1px]',
            // Invalid states
            'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive aria-invalid:outline aria-invalid:border-c-p-50',
            // Disabled states
            'disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50',
            // Responsive
            'md:text-sm',
            className,
          )}
          {...props}
        />
        {showRequiredMarker && (
          <span
            className="pointer-events-none absolute left-[2.7rem] top-1/2 -translate-y-1/2 text-post-required-marker"
            aria-hidden="true"
          >
            *
          </span>
        )}
      </div>
    );
  },
);

ThemedInput.displayName = 'ThemedInput';

export { ThemedInput };
