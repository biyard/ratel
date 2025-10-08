'use client';
import SwitchButton from '@/components/switch-button';

export default function LabelSwitchButton({
  label,
  bgColor,
  textColor,
  value,
  onChange,
}: {
  label: string;
  bgColor: string;
  textColor: string;
  value: boolean;
  onChange: (val: boolean) => void;
}) {
  return (
    <label className="flex items-center cursor-pointer gap-2 select-none">
      <span
        className={`font-medium text-[15px]/[24px] ${value ? textColor : 'text-gray-400'}`}
      >
        {label}
      </span>
      <SwitchButton value={value} onChange={onChange} color={bgColor} />
    </label>
  );
}
