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
    <div
      className={cn(
        'flex flex-row items-start text-sm font-normal gap-2.25 select-none',
        disabled ? 'text-c-wg-200' : 'text-white',
      )}
    >
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
          htmlFor={id}
          className={cn(
            'flex items-center justify-center w-4.25 h-4.25',
            isRounded ? 'rounded-full' : 'rounded-sm',
            disabled
              ? [
                  'cursor-not-allowed border-c-wg-200 bg-c-wg-900',
                  value &&
                    '[&_svg_path]:stroke-c-wg-300 bg-neutral-500 border-neutral-500 light:bg-neutral-300 light:border-neutral-300 [&_svg_path]:stroke-bg',
                ]
              : [
                  'cursor-pointer border border-c-wg-50 light:border-modal-card-border',
                  value && 'bg-primary border-primary [&_svg_path]:stroke-bg',
                ],
          )}
        >
          <CheckboxIcon
            width={13}
            height={9}
            className="[&_path]:stroke-transparent"
          />
        </label>
      </div>

      <div
        className={cn('cursor-pointer', disabled && 'cursor-not-allowed')}
        onClick={handleChange}
        data-testid={id ? `${id}-label` : undefined}
      >
        {children}
      </div>
    </div>
  );
};
