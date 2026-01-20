import { Editor } from '@tiptap/core';

/**
 * Enabled features configuration for the toolbar
 */
export interface EnabledFeatures {
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  strike?: boolean;
  textColor?: boolean;
  highlight?: boolean;
  heading?: boolean;
  align?: boolean;
  alignCenter?: boolean;
  alignRight?: boolean;
  lists?: boolean;
  link?: boolean;
  image?: boolean;
  video?: boolean;
  indent?: boolean;
  table?: boolean;
  pdf?: boolean;
}

export type UploadResult = { url: string };

/**
 * Props for the TiptapEditor component
 */
export interface TiptapEditorProps {
  // Basic editor props
  isMe?: boolean;
  isFoldable?: boolean;
  content?: string;
  onUpdate?: (content: string) => void;
  editable?: boolean;
  placeholder?: string;

  // Styling variant
  variant?: 'default' | 'post';

  // Toolbar customization
  showToolbar?: boolean;
  toolbarPosition?: 'top' | 'bottom';
  enabledFeatures?: EnabledFeatures;
  showBubbleToolbar?: boolean;
  bubbleEnabledFeatures?: EnabledFeatures;
  bubbleToolbarClassName?: string;
  toolbarFooter?: React.ReactNode;

  // Styling
  className?: string;
  toolbarClassName?: string;
  editorClassName?: string;
  minHeight?: string;
  maxHeight?: string;
  overlay?: React.ReactNode;

  // Focus state
  onFocus?: () => void;
  onBlur?: () => void;
  onImageUpload?: (imageUrl: string) => Promise<void>;
  onUploadPDF?: (files: FileList | File[]) => void;
  // Test identifier

  uploadAsset?: (file: File) => Promise<UploadResult>;
  uploadVideo?: (file: File) => Promise<UploadResult>;

  maxImageSizeMB?: number;
  maxVideoSizeMB?: number;

  'data-pw'?: string;
}

/**
 * Props for the TiptapToolbar component
 */
export interface TiptapToolbarProps {
  editor: Editor | null;
  enabledFeatures?: EnabledFeatures;
  className?: string;
  variant?: 'default' | 'post';
  mode?: 'default' | 'bubble';
  openVideoPicker?: () => void;
  onImageUpload?: (imageUrl: string) => Promise<void>;
  onUploadPDF?: (files: FileList | File[]) => void;
  dropdownPortalContainer?: HTMLElement | null;
  onHeadingDropdownOpenChange?: (open: boolean) => void;
  onHeadingDropdownTriggerPointerDown?: () => void;
  headingDropdownContentProps?: import('@/components/ui/dropdown-menu').DropdownMenuContentProps;
  onColorPickerOpenChange?: (open: boolean) => void;
  onColorPickerTriggerPointerDown?: () => void;
}

/**
 * Props for the ToolbarButton component
 */
export interface ToolbarButtonProps
  extends React.HTMLAttributes<HTMLButtonElement> {
  icon: React.ReactNode;
  onClick: () => void;
  active?: boolean;
  disabled?: boolean;
  tooltip?: string;
  className?: string;
  'aria-label'?: string;
}

/**
 * Props for the ColorPicker component
 */
export interface ColorPickerProps {
  type: 'text' | 'background';
  currentColor?: string;
  onColorChange: (color: string) => void;
  disabled?: boolean;
  icon?: React.ReactNode | React.ElementType;
  portalled?: boolean;
  container?: HTMLElement | null;
  contentProps?: import('@/components/ui/dropdown-menu').DropdownMenuContentProps;
  onOpenChange?: (open: boolean) => void;
  onTriggerPointerDown?: () => void;
}

/**
 * Props for the HeadingDropdown component
 */
export interface HeadingDropdownProps {
  editor: Editor | null;
  disabled?: boolean;
  portalled?: boolean;
  container?: HTMLElement | null;
  onOpenChange?: (open: boolean) => void;
  onTriggerPointerDown?: () => void;
  contentProps?: React.ComponentProps<
    typeof import('@radix-ui/react-dropdown-menu').Content
  >;
}

/**
 * Default enabled features
 */
export const DEFAULT_ENABLED_FEATURES: EnabledFeatures = {
  bold: true,
  italic: true,
  underline: true,
  strike: true,
  textColor: true,
  highlight: true,
  heading: true,
  align: true,
  lists: true,
  link: true,
  image: true, // Enabled
  video: true,
  indent: false, // Disabled for now
  table: true, // Enabled
  pdf: true,
};
