import * as React from 'react';

import { cn } from '@/lib/utils';

export interface ThemedTextareaProps extends React.ComponentProps<'textarea'> {
  /**
   * Visual variant of the textarea
   * - 'default': Standard textarea styling
   * - 'post': Uses post creation theme colors (#101010 dark, #a3a3a3 light)
   */
  variant?: 'default' | 'post';
}

/**
 * Themed textarea component with seamless light/dark mode support
 * Uses theme color variables for consistent appearance
 */
const ThemedTextarea = React.forwardRef<
  HTMLTextAreaElement,
  ThemedTextareaProps
>(({ className, variant = 'default', ...props }, ref) => {
  const variantClasses = {
    default:
      'border-input-box-border bg-input-box-bg focus-visible:border-ring focus-visible:ring-ring/50',
    post: '!bg-post-input-bg !border-post-input-border focus-visible:border-post-input-border focus-visible:ring-post-input-border/30',
  };

  return (
    <textarea
      ref={ref}
      data-slot="textarea"
      className={cn(
        // Base styles
        'flex min-h-[60px] w-full rounded-md px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none',
        'text-text-primary placeholder:text-gray-600',
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
        // Prevent resize handle in some browsers if needed
        'resize-y',
        className,
      )}
      {...props}
    />
  );
});

ThemedTextarea.displayName = 'ThemedTextarea';

export { ThemedTextarea };
