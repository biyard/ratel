import { Editor } from '@tiptap/core';

/**
 * Extracts all Base64 image sources from the editor content
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
 * Replaces a Base64 image URL with an S3 URL in the editor content
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
 */
export type UploadImageCallback = (base64: string) => Promise<string>;

/**
 * Uploads all Base64 images to S3 and replaces them in the editor
 * @param editor - The Tiptap editor instance
 * @param uploadCallback - Async function that uploads a Base64 image and returns the S3 URL
 */
export const uploadAndReplaceImages = async (
  editor: Editor,
  uploadCallback: UploadImageCallback,
): Promise<void> => {
  const base64Images = extractBase64Images(editor);

  if (base64Images.length === 0) {
    return;
  }

  // Upload all images in parallel
  const uploadPromises = base64Images.map(async (base64) => {
    try {
      const s3Url = await uploadCallback(base64);
      return { base64, s3Url };
    } catch (error) {
      console.error('Failed to upload image:', error);
      return { base64, s3Url: null };
    }
  });

  const results = await Promise.all(uploadPromises);

  // Create URL map from successful uploads
  const urlMap: Record<string, string> = {};
  results.forEach(({ base64, s3Url }) => {
    if (s3Url) {
      urlMap[base64] = s3Url;
    }
  });

  // Replace all Base64 URLs with S3 URLs
  replaceAllBase64WithUrls(editor, urlMap);
};
