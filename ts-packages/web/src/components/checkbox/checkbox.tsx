'use client';
import CheckboxIcon from '@/assets/icons/checkbox-icon.svg?react';

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
          className={
            isRounded
              ? 'border border-c-wg-50 rounded-full peer-checked:bg-primary peer-checked:border-primary flex items-center justify-center w-4.25 h-4.25 cursor-pointer'
              : 'border border-c-wg-50 rounded-[4px] peer-checked:bg-primary peer-checked:border-primary flex items-center justify-center w-4.25 h-4.25 cursor-pointer'
          }
          htmlFor={id}
        >
          {<CheckboxIcon width={13} height={9} />}
        </label>
      </div>

      <div className="cursor-pointer" onClick={handleChange}>
        {children}
      </div>
    </div>
  );
};
