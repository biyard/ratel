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
  indent?: boolean;
  table?: boolean;
}

/**
 * Props for the TiptapEditor component
 */
export interface TiptapEditorProps {
  // Basic editor props
  content?: string;
  onUpdate?: (content: string) => void;
  editable?: boolean;
  placeholder?: string;

  // Toolbar customization
  showToolbar?: boolean;
  toolbarPosition?: 'top' | 'bottom';
  enabledFeatures?: EnabledFeatures;

  // Styling
  className?: string;
  toolbarClassName?: string;
  editorClassName?: string;
  minHeight?: string;
  maxHeight?: string;

  // Focus state
  onFocus?: () => void;
  onBlur?: () => void;

  // Test identifier
  'data-pw'?: string;
}

/**
 * Props for the TiptapToolbar component
 */
export interface TiptapToolbarProps {
  editor: Editor | null;
  enabledFeatures?: EnabledFeatures;
  className?: string;
}

/**
 * Props for the ToolbarButton component
 */
export interface ToolbarButtonProps {
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
}

/**
 * Props for the HeadingDropdown component
 */
export interface HeadingDropdownProps {
  editor: Editor | null;
  disabled?: boolean;
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
  link: false, // Disabled for now
  image: true, // Enabled
  indent: false, // Disabled for now
  table: true, // Enabled
};
