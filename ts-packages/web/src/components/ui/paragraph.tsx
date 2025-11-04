import * as React from 'react';

import { cn } from '@/lib/utils';
import { Slot } from '@radix-ui/react-slot';
import { cva, VariantProps } from 'class-variance-authority';

export const paragraphVariants = cva('text-[17px]/[20px]', {
  variants: {
    variant: {
      strong: 'font-semibold text-text-primary',
      default: '',
    },
  },
  defaultVariants: {
    variant: 'default',
  },
});

export function Paragraph({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<'p'> &
  VariantProps<typeof paragraphVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'p';

  return (
    <Comp
      className={cn(paragraphVariants({ variant, className }))}
      {...props}
    />
  );
}
