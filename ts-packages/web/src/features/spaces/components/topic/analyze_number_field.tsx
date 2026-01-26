import * as React from 'react';
import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';

type AnalyzeNumberFieldProps = {
  label: React.ReactNode;
  hint?: React.ReactNode;

  value: number;
  onValueChange: (next: number) => void;

  min: number;
  max: number;
  clamp: (v: number) => number;

  fallbackOnBlur: number;

  disabled?: boolean;
  className?: string;
};

export function AnalyzeNumberField({
  label,
  hint,
  value,
  onValueChange,
  min,
  max,
  clamp,
  fallbackOnBlur,
  disabled,
  className,
}: AnalyzeNumberFieldProps) {
  const [raw, setRaw] = React.useState<string>(String(value));

  React.useEffect(() => {
    setRaw(String(value));
  }, [value]);

  return (
    <div className={cn('flex flex-col w-full gap-2', className)}>
      <div className="text-sm font-medium text-text-primary">{label}</div>

      <div className="flex flex-row w-full gap-2">
        <Input
          type="number"
          min={min}
          max={max}
          value={raw}
          disabled={disabled}
          onChange={(e) => {
            const nextRaw = e.target.value;
            setRaw(nextRaw);

            if (nextRaw === '') return;

            const n = Number(nextRaw);
            if (!Number.isFinite(n)) return;

            const fixed = clamp(n);
            onValueChange(fixed);
            if (fixed !== n) setRaw(String(fixed));
          }}
          onBlur={() => {
            const n = Number(raw);
            if (!Number.isFinite(n)) {
              const fixed = clamp(fallbackOnBlur);
              onValueChange(fixed);
              setRaw(String(fixed));
              return;
            }

            const fixed = clamp(n);
            onValueChange(fixed);
            setRaw(String(fixed));
          }}
          className={cn(
            'h-10 w-full rounded-md border bg-background px-3 text-sm text-text-primary',
            'focus:outline-none focus:ring-2 focus:ring-primary/30',
          )}
        />
      </div>

      {hint != null && (
        <div className="text-xs text-muted-foreground">{hint}</div>
      )}
    </div>
  );
}
