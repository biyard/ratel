import * as React from 'react';

import { cn } from '@/lib/utils';

function Col({ children, className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div className={cn('w-full flex flex-col gap-2.5', className)} {...props}>
      {children}
    </div>
  );
}

export { Col };
