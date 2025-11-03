import * as React from 'react';

import { cn } from '@/lib/utils';
import { Slot } from '@radix-ui/react-slot';
import { cva, VariantProps } from 'class-variance-authority';

export const colVariants = cva('w-full flex flex-col gap-2.5', {
  variants: {
    mainAxisAlignment: {
      start: 'justify-start',
      center: 'justify-center',
      end: 'justify-end',
      between: 'justify-between',
    },
    crossAxisAlignment: {
      start: 'items-start',
      center: 'items-center',
      end: 'items-end',
      stretch: 'items-stretch',
    },
  },
});

export function Col({
  className,
  mainAxisAlignment,
  crossAxisAlignment,
  asChild = false,
  ...props
}: React.ComponentProps<'div'> &
  VariantProps<typeof colVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'div';

  return (
    <Comp
      className={cn(
        colVariants({ mainAxisAlignment, crossAxisAlignment, className }),
      )}
      {...props}
    />
  );
}
