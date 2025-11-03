import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

export const headingVariants = cva(
  'leading-tight tracking-tight text-text-primary',
  {
    variants: {
      variant: {
        heading6: 'font-semibold text-sm md:text-base lg:text-lg',
        heading5: 'font-semibold text-base md:text-lg lg:text-xl',
        heading4: 'font-semibold text-lg md:text-xl lg:text-2xl',
        heading3: 'font-semibold text-xl md:text-2xl lg:text-3xl',
        heading2: 'font-bold text-2xl md:text-3xl lg:text-4xl',
        heading1: 'font-bold text-3xl md:text-4xl lg:text-5xl',
      },
    },
    defaultVariants: {
      variant: 'heading1',
    },
  },
);

export default function Heading({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<'h1'> &
  VariantProps<typeof headingVariants> & {
    asChild?: boolean;
  }) {
  let baseComp = 'h1';
  if (variant === 'heading2') {
    baseComp = 'h2';
  } else if (variant === 'heading3') {
    baseComp = 'h3';
  } else if (variant === 'heading4') {
    baseComp = 'h4';
  } else if (variant === 'heading5') {
    baseComp = 'h5';
  } else if (variant === 'heading6') {
    baseComp = 'h6';
  }

  const Comp = asChild ? Slot : baseComp;

  return (
    <>
      <Comp
        data-slot={baseComp}
        className={cn(headingVariants({ variant, className }))}
        {...props}
      />
    </>
  );
}
