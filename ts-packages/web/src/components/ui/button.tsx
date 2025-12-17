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
          'bg-btn-primary-bg text-btn-primary-text border-btn-primary-outline hover:bg-btn-primary-hover-bg hover:border-btn-primary-hover-outline hover:text-btn-primary-hover-text disabled:bg-primary-btn-disable-bg disabled:border-btn-primary-disable-outline disabled:text-btn-primary-disable-text rounded-full',
        rounded_secondary:
          'bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary--disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text rounded-full',
        default:
          'bg-white text-black rounded-xl hover:shadow-[inset_0_0_0_1000px_rgba(0,0,0,0.2)]',
        primary:
          'bg-primary text-black rounded-xl hover:shadow-[inset_0_0_0_1000px_rgba(0,0,0,0.2)]',
        outline:
          'bg-btn-outline-bg text-btn-outline-text border-btn-outline-outline hover:bg-btn-outline-hover-bg hover:border-btn-outline-hover-outline hover:text-btn-outline-hover-text disabled:bg-btn-outline-disable-bg disabled:border-btn-outline-disable-outline disabled:text-btn-outline-disable-text rounded-full',
        text: 'px-10 text-base font-bold bg-transparent transition-colors hover:text-white min-w-[50px] py-[14.5px] text-neutral-400',
      },
      size: {
        default: 'h-auto px-5 py-2.5',
        sm: 'h-auto px-4 py-2 text-xs',
        lg: 'h-auto px-6 py-3 text-base',
      },
      platform: {
        web: 'web',
        mobile: 'mobile',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
      platform: 'web',
    },
  },
);

function Button({
  className,
  variant,
  size,
  platform,
  asChild = false,
  ...props
}: React.ComponentProps<'button'> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'button';

  return (
    <Comp
      data-platform={platform}
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, buttonVariants };
