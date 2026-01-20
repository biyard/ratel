import { createElement, isValidElement, useMemo, useState } from 'react';
import { cn } from '@/lib/utils';
import { ColorPickerProps } from './types';
import { EditorPaint, EditorPaint2 } from '../icons';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

const PRESET_COLORS = [
  { name: 'Black', value: '#000000' },
  { name: 'Gray', value: '#737373' },
  { name: 'Red', value: '#ef4444' },
  { name: 'Orange', value: '#f97316' },
  { name: 'Yellow', value: '#fcb300' },
  { name: 'Green', value: '#22c55e' },
  { name: 'Blue', value: '#3b82f6' },
  { name: 'Purple', value: '#a855f7' },
  { name: 'Pink', value: '#db2780' },
  { name: 'Cyan', value: '#6eedd8' },
];

export const ColorPicker = ({
  type,
  currentColor,
  onColorChange,
  disabled = false,
  icon,
  portalled = true,
  container,
  contentProps,
  onOpenChange,
  onTriggerPointerDown,
}: ColorPickerProps) => {
  const [customColor, setCustomColor] = useState(currentColor || '#000000');
  const [open, setOpen] = useState(false);

  const close = () => {
    setOpen(false);
    onOpenChange?.(false);
  };

  const handleColorSelect = (color: string) => {
    onColorChange(color);
    setCustomColor(color);
    close();
  };

  const handleCustomColorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const color = e.target.value;
    setCustomColor(color);
    onColorChange(color);
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    // Prevent losing text selection when clicking color picker
    e.preventDefault();
    e.stopPropagation();
    onTriggerPointerDown?.();
    if (disabled) return;
    if (!open) {
      setOpen(true);
      onOpenChange?.(true);
    }
  };

  const iconContent = useMemo(() => {
    const fallback = type === 'text' ? <EditorPaint /> : <EditorPaint2 />;
    const value = icon ?? fallback;

    if (isValidElement(value)) {
      return value;
    }

    if (typeof value === 'string' || typeof value === 'function') {
      return createElement(value as React.ElementType);
    }

    return value;
  }, [icon, type]);

  return (
    <DropdownMenu
      modal={false}
      open={open}
      onOpenChange={(nextOpen) => {
        if (nextOpen === open) return;
        setOpen(nextOpen);
        onOpenChange?.(nextOpen);
      }}
    >
      <DropdownMenuTrigger asChild>
        <button
          tabIndex={-1}
          type="button"
          disabled={disabled}
          onMouseDown={handleMouseDown}
          aria-label={type === 'text' ? 'Text color' : 'Background color'}
          title={type === 'text' ? 'Text color' : 'Background color'}
          className={cn(
            'flex items-center justify-center',
            'w-6 h-6 rounded transition-all',
            'focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-1',
            '[&_svg]:w-5 [&_svg]:h-5',
            'data-[state=open]:[&_svg]:text-primary data-[state=open]:[&_svg_path]:fill-primary',
            '[&_svg]:text-foreground-muted [&_svg_path]:fill-foreground-muted',
            'data-[state=open]:bg-primary/10 data-[state=open]:border data-[state=open]:border-primary/20',
            'bg-transparent hover:bg-accent-hover border border-transparent',
            disabled && 'opacity-50 cursor-not-allowed hover:bg-transparent',
          )}
        >
          {iconContent}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="start"
        className="p-3 w-56"
        portalled={portalled}
        container={container}
        {...contentProps}
      >
        <div className="mb-3">
          <p className="mb-2 text-xs font-medium text-foreground-muted">
            {type === 'text' ? 'Text Color' : 'Highlight Color'}
          </p>
          <div className="grid grid-cols-5 gap-2">
            {PRESET_COLORS.map((color) => (
              <button
                key={color.value}
                type="button"
                onClick={() => handleColorSelect(color.value)}
                className={cn(
                  'w-8 h-8 rounded border-2 transition-all',
                  'hover:scale-110 focus:outline-none focus:ring-2 focus:ring-primary',
                  currentColor === color.value
                    ? 'border-primary'
                    : 'border-transparent',
                )}
                style={{ backgroundColor: color.value }}
                title={color.name}
                aria-label={color.name}
              />
            ))}
          </div>
        </div>

        <div className="pt-3 border-t border-border">
          <label className="block mb-2 text-xs font-medium text-foreground-muted">
            Custom Color
          </label>
          <div className="flex gap-2 items-center">
            <input
              type="color"
              value={customColor}
              onChange={handleCustomColorChange}
              className="w-10 h-10 rounded border cursor-pointer border-border"
            />
            <input
              type="text"
              value={customColor}
              onChange={(e) => {
                setCustomColor(e.target.value);
              }}
              onBlur={(e) => {
                if (/^#[0-9A-Fa-f]{6}$/.test(e.target.value)) {
                  onColorChange(e.target.value);
                }
              }}
              placeholder="#000000"
              className={cn(
                'flex-1 px-2 py-1.5 text-sm rounded',
                'bg-input border border-border',
                'text-foreground placeholder:text-foreground-muted',
                'focus:outline-none focus:ring-2 focus:ring-primary',
              )}
            />
          </div>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
