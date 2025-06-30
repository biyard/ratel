import React from 'react';
import { Button } from '../ui/button';
import Loading from '@/app/loading';
import { cn } from '@/lib/utils';
interface PrimaryButtonProps {
  disabled: boolean;
  onClick: () => void;
  children?: React.ReactNode;
}

export const PrimaryButton = ({
  disabled,
  onClick,
  children,
}: PrimaryButtonProps) => {
  const className = !disabled
    ? 'w-full px-10 py-5 rounded-[10px] bg-[#fcb300] hover:bg-[#ca8f00] text-black text-bold text-[16px] hover:text-black cursor-pointer'
    : 'w-full px-5 py-5 rounded-[10px] bg-[#777677] text-[#aeaeae] text-bold text-[16px] cursor-not-allowed';

  return (
    <button className={className} disabled={disabled} onClick={onClick}>
      {children}
    </button>
  );
};

interface LoadablePrimaryButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  isLoading: boolean;
}

export const LoadablePrimaryButton = ({
  isLoading,
  children,
  disabled,
  className,
  ...props
}: LoadablePrimaryButtonProps) => {
  const finalClassName = disabled
    ? cn(className, 'opacity-50 cursor-not-allowed')
    : className;
  return (
    <Button
      variant="rounded_primary"
      className={finalClassName}
      disabled={disabled || isLoading}
      {...props}
    >
      {isLoading ? <Loading className="w-10 h-5" /> : children}
    </Button>
  );
};
