import { Editor } from '@tiptap/core';

/**
 * Image data structure with src and optional filename
 */
export interface ImageData {
  src: string;
  alt?: string;
  title?: string;
}

/**
 * Extracts all Base64 image sources from the editor content
 * @deprecated Use extractBase64ImagesWithMetadata for full metadata support
 */
export const extractBase64Images = (editor: Editor): string[] => {
  const html = editor.getHTML();
  const base64Pattern = /<img[^>]+src="(data:image\/[^"]+)"/g;
  const matches: string[] = [];
  let match;

  while ((match = base64Pattern.exec(html)) !== null) {
    matches.push(match[1]);
  }

  return matches;
};

/**
 * Extracts all Base64 images with their metadata (alt, title) from the editor
 * This traverses the editor's document tree to get accurate node attributes
 */
export const extractBase64ImagesWithMetadata = (
  editor: Editor,
): ImageData[] => {
  const images: ImageData[] = [];
  const { doc } = editor.state;

  doc.descendants((node) => {
    if (
      node.type.name === 'image' &&
      node.attrs.src &&
      node.attrs.src.startsWith('data:image/')
    ) {
      images.push({
        src: node.attrs.src,
        alt: node.attrs.alt || undefined,
        title: node.attrs.title || undefined,
      });
    }
  });

  return images;
};

/**
 * Replaces a Base64 image URL with an S3 URL in the editor content
 * Preserves alt and title attributes
 */
export const replaceBase64WithUrl = (
  editor: Editor,
  base64Src: string,
  s3Url: string,
): void => {
  const { state, view } = editor;
  const { doc } = state;
  let tr = state.tr;

  doc.descendants((node, pos) => {
    if (node.type.name === 'image' && node.attrs.src === base64Src) {
      tr = tr.setNodeMarkup(pos, undefined, {
        ...node.attrs,
        src: s3Url,
        // Preserve alt and title attributes
      });
    }
  });

  view.dispatch(tr);
};

/**
 * Replaces all Base64 images with S3 URLs using a mapping object
 * @param editor - The Tiptap editor instance
 * @param urlMap - A map of Base64 URLs to S3 URLs
 */
export const replaceAllBase64WithUrls = (
  editor: Editor,
  urlMap: Record<string, string>,
): void => {
  const { state, view } = editor;
  const { doc } = state;
  let tr = state.tr;

  doc.descendants((node, pos) => {
    if (
      node.type.name === 'image' &&
      node.attrs.src.startsWith('data:image/')
    ) {
      const s3Url = urlMap[node.attrs.src];
      if (s3Url) {
        tr = tr.setNodeMarkup(pos, undefined, {
          ...node.attrs,
          src: s3Url,
        });
      }
    }
  });

  view.dispatch(tr);
};

/**
 * Upload callback type for S3 upload
 * Takes base64 string and optional filename, returns S3 URL
 */
export type UploadImageCallback = (
  base64: string,
  filename?: string,
) => Promise<string>;

/**
 * Converts a base64 data URL to a Blob
 * @param base64 - The base64 data URL (e.g., "data:image/png;base64,...")
 * @returns A Blob object
 */
export const base64ToBlob = (base64: string): Blob => {
  const parts = base64.split(',');
  const mime = parts[0].match(/:(.*?);/)?.[1] || 'image/png';
  const bstr = atob(parts[1]);
  let n = bstr.length;
  const u8arr = new Uint8Array(n);
  while (n--) {
    u8arr[n] = bstr.charCodeAt(n);
  }
  return new Blob([u8arr], { type: mime });
};

/**
 * Converts a base64 data URL to a File object
 * @param base64 - The base64 data URL
 * @param filename - The filename for the file
 * @returns A File object
 */
export const base64ToFile = (base64: string, filename: string): File => {
  const blob = base64ToBlob(base64);
  return new File([blob], filename, { type: blob.type });
};

/**
 * Uploads all Base64 images to S3 and replaces them in the editor
 * Preserves image metadata (alt, title) from the original images
 * @param editor - The Tiptap editor instance
 * @param uploadCallback - Async function that uploads a Base64 image and returns the S3 URL
 * @returns Array of successfully uploaded S3 URLs
 */
export const uploadAndReplaceImages = async (
  editor: Editor,
  uploadCallback: UploadImageCallback,
): Promise<string[]> => {
  const images = extractBase64ImagesWithMetadata(editor);

  if (images.length === 0) {
    return [];
  }

  // Upload all images in parallel
  const uploadPromises = images.map(async (imageData) => {
    try {
      // Pass filename from alt attribute if available
      const filename = imageData.alt || imageData.title;
      const s3Url = await uploadCallback(imageData.src, filename);
      return { imageData, s3Url };
    } catch (error) {
      console.error('Failed to upload image:', error);
      return { imageData, s3Url: null };
    }
  });

  const results = await Promise.all(uploadPromises);

  // Replace images in the editor
  const { state, view } = editor;
  const { doc } = state;
  let tr = state.tr;

  // Build a map for quick lookup and collect successful URLs
  const urlMap = new Map<string, string>();
  const uploadedUrls: string[] = [];

  results.forEach(({ imageData, s3Url }) => {
    if (s3Url) {
      urlMap.set(imageData.src, s3Url);
      uploadedUrls.push(s3Url);
    }
  });

  // Replace all base64 images with S3 URLs while preserving metadata
  doc.descendants((node, pos) => {
    if (
      node.type.name === 'image' &&
      node.attrs.src &&
      node.attrs.src.startsWith('data:image/')
    ) {
      const s3Url = urlMap.get(node.attrs.src);
      if (s3Url) {
        tr = tr.setNodeMarkup(pos, undefined, {
          ...node.attrs,
          src: s3Url,
          // alt and title are preserved from node.attrs
        });
      }
    }
  });

  view.dispatch(tr);

  return uploadedUrls;
};
