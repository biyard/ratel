import React from 'react';

interface SwitchProps {
  checked: boolean;
  onChange: () => void;
}

export default function Switch({ checked, onChange }: SwitchProps) {
  return (
    <button
      type="button"
      onClick={onChange}
      className={`${
        checked ? 'bg-[#2563eb]' : 'bg-[#bfbfbf]'
      } relative inline-flex items-center justify-start h-[16px] w-[28px] rounded-full transition-colors duration-200 ease-in-out focus:outline-none p-[2px]`}
    >
      <span
        className={`${
          checked ? 'translate-x-[10px]' : 'translate-x-[2px]'
        } inline-block h-full w-[12px] transform rounded-full bg-white transition-transform duration-200 ease-in-out`}
      />
    </button>
  );
}
