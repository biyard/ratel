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
  const handleClick = () => {
    if (!disabled) {
      onChange(!value);
    }
  };

  return (
    <div
      className="flex flex-row items-start text-sm font-normal text-white cursor-pointer gap-2.25 select-none"
      onClick={handleClick}
    >
      <div className="flex relative flex-row justify-start items-center cursor-pointer gap-[6px]">
        <input
          id={id}
          disabled={disabled}
          type="checkbox"
          className="hidden peer"
          checked={value}
          onChange={() => {}} // Controlled component, onChange handled by parent div
          readOnly
        />

        <label
          className={
            isRounded
              ? 'border border-c-wg-50 rounded-full peer-checked:bg-primary peer-checked:border-primary flex items-center justify-center w-4.25 h-4.25 cursor-pointer'
              : 'border border-c-wg-50 rounded-[4px] peer-checked:bg-primary peer-checked:border-primary flex items-center justify-center w-4.25 h-4.25 cursor-pointer'
          }
        >
          {<CheckboxIcon width={13} height={9} />}
        </label>
      </div>

      {children}
    </div>
  );
};
