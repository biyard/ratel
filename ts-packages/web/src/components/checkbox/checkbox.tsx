'use client';
import CheckboxIcon from '@/assets/icons/checkbox-icon.svg?react';
import { cn } from '@/lib/utils';

interface CheckboxProps {
  isRounded?: boolean;
  id?: string;
  value?: boolean;
  onChange: (check: boolean) => void;
  children?: React.ReactNode;
  disabled?: boolean;
}

export const Checkbox = ({
  isRounded = false,
  id,
  value,
  onChange,
  children,
  disabled = false,
}: CheckboxProps) => {
  const handleChange = () => {
    if (!disabled) {
      onChange(!value);
    }
  };

  return (
    <div className="flex flex-row items-start text-sm font-normal text-white gap-2.25 select-none">
      <div className="flex relative flex-row justify-start items-center cursor-pointer gap-[6px]">
        <input
          id={id}
          disabled={disabled}
          type="checkbox"
          className="hidden peer"
          checked={value}
          onChange={handleChange}
        />

        <label
          className={cn(
            'border border-c-wg-50',
            'peer-checked:bg-primary peer-checked:border-primary',
            'peer-checked:[&_svg_path]:stroke-bg',
            'flex items-center justify-center',
            'w-4.25 h-4.25 cursor-pointer',
            isRounded ? 'rounded-full' : 'rounded-sm',
          )}
          htmlFor={id}
        >
          <CheckboxIcon
            width={13}
            height={9}
            className="[&_path]:stroke-transparent"
          />
        </label>
      </div>

      <div
        className="cursor-pointer"
        onClick={handleChange}
        data-testid={id ? `${id}-label` : undefined}
      >
        {children}
      </div>
    </div>
  );
};
