import * as React from 'react';
import { Check, ChevronDown, X } from 'lucide-react';

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';

type Option = {
  label: string;
  value: string;
};

type MultiSelectProps = {
  options: Option[];
  value: string[];
  onChange: (value: string[]) => void;
  placeholder?: string;
  className?: string;
  disabled?: boolean;
  maxHeight?: number;
};

export const MultiSelect: React.FC<MultiSelectProps> = ({
  options,
  value,
  onChange,
  placeholder = '',
  className,
  disabled,
  maxHeight = 240,
}) => {
  const [open, setOpen] = React.useState(false);

  const triggerRef = React.useRef<HTMLButtonElement>(null);
  const [triggerWidth, setTriggerWidth] = React.useState<number | null>(null);

  React.useLayoutEffect(() => {
    if (!open) return;

    const el = triggerRef.current;
    if (!el) return;

    const update = () => {
      const w = Math.ceil(el.getBoundingClientRect().width);
      setTriggerWidth(w);
    };

    const raf = requestAnimationFrame(update);

    const ro = new ResizeObserver(update);
    ro.observe(el);

    return () => {
      cancelAnimationFrame(raf);
      ro.disconnect();
    };
  }, [open]);

  const toggleValue = (val: string) => {
    if (value.includes(val)) onChange(value.filter((v) => v !== val));
    else onChange([...value, val]);
  };

  const clearAll = (e: React.MouseEvent) => {
    // e.preventDefault();
    e.stopPropagation();
    onChange([]);
  };

  const selectedOptions = options.filter((opt) => value.includes(opt.value));

  return (
    <Popover
      open={open}
      onOpenChange={(v) => {
        setOpen(v);
        if (!v) setTriggerWidth(null);
      }}
    >
      <PopoverTrigger asChild>
        <Button
          ref={triggerRef}
          type="button"
          variant="outline"
          role="combobox"
          aria-expanded={open}
          disabled={disabled}
          data-testid="multi-select-trigger"
          className={cn(
            'justify-between gap-2 min-w-0',
            !selectedOptions.length && 'text-muted-foreground',
            className,
          )}
        >
          <div className="flex flex-wrap flex-1 gap-1 items-center min-w-0">
            {selectedOptions.length === 0 ? (
              <span className="truncate">{placeholder}</span>
            ) : (
              selectedOptions.map((opt) => (
                <Badge
                  key={opt.value}
                  variant="secondary"
                  className="flex gap-1 items-center max-w-full text-gray-900 bg-primary"
                >
                  <span className="truncate">{opt.label}</span>
                </Badge>
              ))
            )}
          </div>

          <div className="flex gap-1 items-center shrink-0">
            {selectedOptions.length > 0 && (
              <span
                role="button"
                tabIndex={0}
                onPointerDown={clearAll}
                className="p-1 rounded-full hover:bg-muted"
                aria-label="Clear selection"
              >
                <X className="w-3 h-3" />
              </span>
            )}
            <ChevronDown className="w-4 h-4 opacity-60" />
          </div>
        </Button>
      </PopoverTrigger>

      <PopoverContent
        align="start"
        className="p-0 max-w-none min-w-[220px]"
        style={
          {
            width: triggerWidth ? `${triggerWidth}px` : undefined,
          } as React.CSSProperties
        }
      >
        <Command>
          <CommandInput className="text-text-primary" placeholder="검색..." />
          <CommandList>
            <CommandEmpty className="text-gray-500">
              결과가 없습니다.
            </CommandEmpty>
            <CommandGroup>
              <ScrollArea style={{ maxHeight }}>
                {options.map((opt) => {
                  const isSelected = value.includes(opt.value);
                  return (
                    <CommandItem
                      key={opt.value}
                      onSelect={() => toggleValue(opt.value)}
                      data-testid={`multi-select-option-${opt.value}`}
                      className="flex gap-2 justify-between items-center"
                    >
                      <span>{opt.label}</span>
                      {isSelected && <Check className="w-4 h-4 shrink-0" />}
                    </CommandItem>
                  );
                })}
              </ScrollArea>
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
