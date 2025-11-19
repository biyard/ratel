import Title, { type TitleProps } from './title';
import { LinearScaleQuestion } from '@/features/spaces/polls/types/poll-question';
import { useState, useEffect } from 'react';

interface LinearScaleSliderProps extends LinearScaleQuestion, TitleProps {
  disabled?: boolean;
  selectedValue?: number;
  onSelect: (value: number) => void;
}

export default function LinearScaleSlider(props: LinearScaleSliderProps) {
  const {
    min_label,
    min_value,
    max_label,
    max_value,
    selectedValue,
    onSelect,
  } = props;

  const [localValue, setLocalValue] = useState(selectedValue ?? min_value ?? 0);

  useEffect(() => {
    if (selectedValue !== undefined) {
      setLocalValue(selectedValue);
    }
  }, [selectedValue]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseInt(e.target.value, 10);
    setLocalValue(val);
    onSelect(val);
  };

  const minVal = min_value ?? 0;
  const maxVal = max_value ?? 10;
  const range = maxVal - minVal;

  return (
    <div className="flex flex-col gap-4 w-full">
      <Title {...props} />

      <div className="flex flex-col gap-4 px-2">
        <div className="flex flex-col items-center gap-2">
          <div className="text-2xl font-bold text-primary">{localValue}</div>
          <div className="text-xs text-neutral-400">
            {minVal} ~ {maxVal}
          </div>
        </div>

        <div className="relative w-full">
          <input
            type="range"
            min={minVal}
            max={maxVal}
            value={localValue}
            onChange={handleChange}
            className="w-full h-2 bg-neutral-700 rounded-lg appearance-none cursor-pointer
              [&::-webkit-slider-thumb]:appearance-none
              [&::-webkit-slider-thumb]:w-5
              [&::-webkit-slider-thumb]:h-5
              [&::-webkit-slider-thumb]:rounded-full
              [&::-webkit-slider-thumb]:bg-primary
              [&::-webkit-slider-thumb]:cursor-pointer
              [&::-moz-range-thumb]:w-5
              [&::-moz-range-thumb]:h-5
              [&::-moz-range-thumb]:rounded-full
              [&::-moz-range-thumb]:bg-primary
              [&::-moz-range-thumb]:border-0
              [&::-moz-range-thumb]:cursor-pointer"
          />

          <div className="hidden md:flex justify-between mt-2 px-0.5">
            {Array.from({ length: range + 1 }, (_, i) => {
              const val = minVal + i;
              const isSelected = val === localValue;
              return (
                <div
                  key={val}
                  className={`text-xs ${
                    isSelected ? 'text-primary font-bold' : 'text-neutral-500'
                  }`}
                >
                  {val}
                </div>
              );
            })}
          </div>
        </div>

        <div className="flex justify-between gap-4 text-sm text-neutral-400">
          <div className="text-left flex-1">{min_label}</div>
          <div className="text-right flex-1">{max_label}</div>
        </div>

        <div className="flex md:hidden flex-wrap gap-2 justify-center">
          {[minVal, Math.floor((minVal + maxVal) / 2), maxVal].map((val) => (
            <button
              key={val}
              onClick={() => {
                setLocalValue(val);
                onSelect(val);
              }}
              className={`px-4 py-2 rounded-lg text-sm transition-colors ${
                localValue === val
                  ? 'bg-primary text-black font-semibold'
                  : 'bg-neutral-700 text-neutral-200 hover:bg-neutral-600'
              }`}
            >
              {val}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
