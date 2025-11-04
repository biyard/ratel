import * as React from 'react';

import { cn } from '@/lib/utils';
import { cva, VariantProps } from 'class-variance-authority';
import { Slot } from '@radix-ui/react-slot';

export const rowVariants = cva('w-full flex flex-row gap-2.5', {
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

function Row({
  className,
  mainAxisAlignment,
  crossAxisAlignment,
  asChild = false,
  ...props
}: React.ComponentProps<'div'> &
  VariantProps<typeof rowVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'div';

  return (
    <Comp
      className={cn(
        rowVariants({ mainAxisAlignment, crossAxisAlignment, className }),
      )}
      {...props}
    />
  );
}

export { Row };
