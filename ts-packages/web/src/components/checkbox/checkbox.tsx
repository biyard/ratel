'use client';
import { useState } from 'react';
import CheckboxIcon from '@/assets/icons/checkbox-icon.svg?react';

interface CheckboxProps {
  isRounded?: boolean;
  id?: string;
  value?: boolean;
  onChange: (check: boolean) => void;
  children?: React.ReactNode;
}

export const Checkbox = ({
  isRounded = false,
  id,
  onChange,
  value,
  children,
}: CheckboxProps) => {
  const [checked, setChecked] = useState(value || false);

  return (
    <div className="flex flex-row items-start font-normal text-white text-sm/16 gap-2.25">
      <div className="flex relative flex-row justify-start items-center cursor-pointer gap-[6px]">
        <input
          id={id}
          type="checkbox"
          className="hidden peer"
          checked={value}
          onChange={() => {
            const check = checked;
            setChecked(!check);
            onChange(!check);
          }}
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

      {children}
    </div>
  );
};
