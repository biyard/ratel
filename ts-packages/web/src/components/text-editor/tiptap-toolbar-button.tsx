import { cn } from '@/lib/utils';
import { ToolbarButtonProps } from './types';

export const ToolbarButton = ({
  icon,
  onClick,
  active = false,
  disabled = false,
  tooltip,
  className,
  'aria-label': ariaLabel,
  ...props
}: ToolbarButtonProps) => {
  const handleMouseDown = (e: React.MouseEvent) => {
    // Prevent losing text selection when clicking toolbar buttons
    e.preventDefault();
  };

  return (
    <button
      type="button"
      tabIndex={-1}
      onClick={onClick}
      onMouseDown={handleMouseDown}
      disabled={disabled}
      aria-label={ariaLabel || tooltip}
      title={tooltip}
      className={cn(
        // Base styles
        'flex items-center justify-center',
        'size-8 rounded transition-all',
        'focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-1',

        // Icon color - more visible when active
        '[&_svg]:size-6',
        active
          ? '[&_svg]:text-primary [&_svg_path]:fill-primary [&_svg_path]:stroke-primary' // Use primary color for active state
          : '[&_svg]:text-foreground-muted [&_svg_path]:fill-foreground-muted [&_svg_path]:stroke-gray-500 [&_svg_line]:stroke-gray-500 [&_svg_rect]:stroke-gray-500', // Muted color with hover effect

        // Background and hover states with better contrast
        active
          ? 'bg-primary/10 border border-primary/20' // Subtle background + border for active state
          : 'bg-transparent hover:bg-accent-hover border border-transparent',

        // Disabled state
        disabled && 'opacity-50 cursor-not-allowed hover:bg-transparent',

        // Custom className
        className,
      )}
      {...props}
    >
      {icon}
    </button>
  );
};
