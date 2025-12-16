import FileModel, { toFileExtension } from '../types/file';

export function extractFilesFromHtml(html: string): FileModel[] {
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');
  const fileUrls = new Set<string>();

  doc.querySelectorAll('img').forEach((img) => {
    const src = img.src;
    if (src && (isValidFileUrl(src) || isBase64Image(src))) {
      fileUrls.add(src);
    }
  });

  return Array.from(fileUrls).map((url) => urlToFileModel(url));
}

function isValidFileUrl(url: string): boolean {
  try {
    const urlObj = new URL(url);
    return (
      urlObj.protocol === 'https:' &&
      (urlObj.hostname.includes('s3.') ||
        urlObj.hostname.includes('cloudfront.') ||
        urlObj.hostname.includes('rat-ap'))
    );
  } catch {
    return false;
  }
}

function isBase64Image(url: string): boolean {
  return url.startsWith('data:image/');
}

function urlToFileModel(url: string): FileModel {
  if (isBase64Image(url)) {
    const match = url.match(/^data:image\/(\w+);base64,/);
    const extension = match?.[1] || 'png';
    return {
      name: `image.${extension}`,
      size: '0',
      ext: toFileExtension(extension),
      url,
    };
  }

  const fileName = url.split('/').pop()?.split('?')[0] || 'image';
  const extension = fileName.split('.').pop() || 'png';

  return {
    name: fileName,
    size: '0',
    ext: toFileExtension(extension),
    url,
  };
}
