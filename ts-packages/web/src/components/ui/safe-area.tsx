import * as React from 'react';

import { cn } from '@/lib/utils';
import { Slot } from '@radix-ui/react-slot';
import { cva, VariantProps } from 'class-variance-authority';

const safeAreaVariants = cva('mx-auto', {
  variants: {
    variant: {
      default: 'flex flex-col gap-10',
      row: 'flex flex-row gap-10',
    },
  },

  defaultVariants: {
    variant: 'default',
  },
});

export function SafeArea({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<'div'> &
  VariantProps<typeof safeAreaVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'div';

  return (
    <Comp className={cn(safeAreaVariants({ variant, className }))} {...props} />
  );
}
