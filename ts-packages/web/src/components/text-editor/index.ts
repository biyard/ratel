// Main editor components
export { TiptapEditor } from './tiptap-editor';

// Image utilities for S3 upload
export {
  extractBase64Images,
  replaceBase64WithUrl,
  replaceAllBase64WithUrls,
  uploadAndReplaceImages,
  type UploadImageCallback,
} from './image-utils';

// Types
export type {
  EnabledFeatures,
  TiptapToolbarProps,
  ToolbarButtonProps,
  ColorPickerProps,
  HeadingDropdownProps,
} from './types';

export { DEFAULT_ENABLED_FEATURES } from './types';
