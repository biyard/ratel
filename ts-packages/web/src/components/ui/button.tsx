import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

const buttonVariants = cva(
  'inline-flex items-center justify-center gap-2.5 whitespace-nowrap font-bold text-sm transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-[15px] shrink-0 [&_svg]:shrink-0 outline-none focus-visible:ring-2 focus-visible:ring-offset-2 font-[var(--font-raleway)]',
  {
    variants: {
      variant: {
        rounded_primary:
          'bg-primary text-background rounded-full hover:bg-primary/70 hover:shadow-[inset_0_0_0_1000px_rgba(0,0,0,0.2)]',
        rounded_secondary:
          'bg-white text-background rounded-full hover:bg-white/80 hover:shadow-[inset_0_0_0_1000px_rgba(0,0,0,0.2)]',
        default:
          'bg-white text-black rounded-xl hover:bg-white/80 hover:shadow-[inset_0_0_0_1000px_rgba(0,0,0,0.2)]',
        outline:
          'bg-transparent text-white rounded-lg border border-wg-70 hover:border-white hover:bg-background',
      },
      size: {
        default: 'h-auto px-5 py-2.5',
        sm: 'h-auto px-4 py-2 text-xs',
        lg: 'h-auto px-6 py-3 text-base',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
);

function Button({
  className,
  variant,
  size,
  asChild = false,
  ...props
}: React.ComponentProps<'button'> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'button';

  return (
    <Comp
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, buttonVariants };
