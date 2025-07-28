'use client';
import { Input } from '@/components/ui/input';
import React from 'react';
import { ShapeArrowDown } from '@/components/icons';

export default function LinearScaleSelection({
  minValue,
  maxValue,
  setMaxValue,
  labels,
  setLabels,
}: {
  minValue: number;
  maxValue: number;
  setMaxValue: (val: number) => void;
  labels: Record<number, string>;
  setLabels: (val: number, label: string) => void;
}) {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-row items-center gap-2">
        <div className="bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 text-white text-sm text-center min-w-20 ">
          {minValue}
        </div>
        <span className="text-white text-sm">~</span>
        <div className="relative inline-block min-w-20">
          <select
            className="appearance-none bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 text-white text-sm w-full"
            value={maxValue}
            onChange={(e) => {
              const val = Number(e.target.value);
              setMaxValue(val);
            }}
          >
            {Array.from({ length: 9 }, (_, i) => {
              const value = i + 2;
              return (
                <option key={`max-${value}`} value={value}>
                  {value}
                </option>
              );
            })}
          </select>
          <ShapeArrowDown className="pointer-events-none absolute right-3 top-1/2 transform -translate-y-1/2 text-neutral-500 w-5 h-5" />
        </div>
      </div>

      <div className="flex flex-col justify-start items-start">
        <div className="flex flex-row items-center justify-start gap-5 w-80 mb-3">
          <span className="font-medium text-white text-sm w-5">{minValue}</span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 rounded-none"
            placeholder="Label"
            value={labels[minValue] || ''}
            onChange={(e) => {
              const val = e.target.value;
              setLabels(minValue, val);
            }}
          />
        </div>

        <div className="flex flex-row items-center justify-start gap-5 w-80">
          <span className="font-medium text-white text-sm w-5">{maxValue}</span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 rounded-none"
            placeholder="Label"
            value={labels[maxValue] || ''}
            onChange={(e) => {
              const val = e.target.value;
              setLabels(maxValue, val);
            }}
          />
        </div>
      </div>
    </div>
  );
}
