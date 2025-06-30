'use client';
import React, { useState } from 'react';
import CheckboxIcon from '@/assets/icons/checkbox-icon.svg';

interface CheckboxProps {
  isRounded?: boolean;
  id?: string;
  onChange: (check: boolean) => void;
  children?: React.ReactNode;
}

export const Checkbox = ({
  isRounded = false,
  id,
  onChange,
  children,
}: CheckboxProps) => {
  const [checked, setChecked] = useState(false);
  return (
    <div className="text-white text-sm/16 font-normal flex flex-row gap-2.25 items-start">
      <div className="relative flex flex-row items-center justify-start gap-[6px] cursor-pointer">
        <input
          id={id}
          type="checkbox"
          className="peer hidden"
          checked={checked}
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
