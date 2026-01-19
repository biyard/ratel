import { cn } from '@/lib/utils';
import { HeadingDropdownProps } from './types';
import { EditorParagraph, EditorArrDown } from '../icons';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

const HEADING_OPTIONS = [
  { value: 'paragraph', label: 'Normal', className: 'text-sm' },
  { value: '1', label: 'Heading 1', className: 'text-2xl font-bold' },
  { value: '2', label: 'Heading 2', className: 'text-xl font-bold' },
  { value: '3', label: 'Heading 3', className: 'text-lg font-semibold' },
] as const;

export const HeadingDropdown = ({
  editor,
  disabled = false,
  portalled = true,
  container,
  onOpenChange,
  onTriggerPointerDown,
  contentProps,
}: HeadingDropdownProps) => {
  const getCurrentHeading = () => {
    if (!editor) return 'Normal';
    if (editor.isActive('heading', { level: 1 })) return 'H1';
    if (editor.isActive('heading', { level: 2 })) return 'H2';
    if (editor.isActive('heading', { level: 3 })) return 'H3';
    return 'Normal';
  };

  const handleSelect = (value: string) => {
    if (!editor) return;

    const chain = editor.chain().focus();

    if (value === 'paragraph') {
      chain.setParagraph().run();
    } else {
      chain.setHeading({ level: parseInt(value) as 1 | 2 | 3 }).run();
    }
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    // Prevent losing text selection when clicking heading dropdown
    e.preventDefault();
    onTriggerPointerDown?.();
  };

  return (
    <DropdownMenu onOpenChange={onOpenChange}>
      <DropdownMenuTrigger asChild>
        <button
          tabIndex={-1}
          type="button"
          disabled={disabled}
          onMouseDown={handleMouseDown}
          className={cn(
            'flex items-center gap-1.5 px-2 py-1',
            'rounded transition-all',
            'focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-1',
            'data-[state=open]:bg-primary/10 data-[state=open]:border data-[state=open]:border-primary/20',
            'bg-transparent hover:bg-accent-hover border border-transparent',
            disabled && 'opacity-50 cursor-not-allowed hover:bg-transparent',
          )}
        >
          <EditorParagraph
            className={cn(
              'w-5 h-5',
              'text-foreground-muted [&_path]:fill-foreground-muted',
              'data-[state=open]:text-primary data-[state=open]:[&_path]:fill-primary',
            )}
          />
          <span className="text-sm font-medium text-left text-foreground min-w-[60px]">
            {getCurrentHeading()}
          </span>
          <EditorArrDown
            className={cn(
              'w-3 h-3 transition-transform',
              'text-foreground-muted [&_path]:fill-foreground-muted',
              'data-[state=open]:text-primary data-[state=open]:rotate-180 data-[state=open]:[&_path]:fill-primary',
            )}
          />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="start"
        className="w-48"
        portalled={portalled}
        container={container}
        {...contentProps}
      >
        {HEADING_OPTIONS.map((option) => (
          <DropdownMenuItem
            key={option.value}
            onClick={() => handleSelect(option.value)}
            className={cn('cursor-pointer', option.className)}
          >
            {option.label}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
