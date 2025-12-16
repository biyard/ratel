import FileModel, { toFileExtension } from '../types/file';

export function extractFilesFromHtml(html: string): FileModel[] {
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');
  const fileUrls = new Set<string>();

  doc.querySelectorAll('img').forEach((img) => {
    const src = img.src;
    if (src && isValidFileUrl(src)) {
      fileUrls.add(src);
    }
  });

  doc.querySelectorAll('video source, video').forEach((video) => {
    const src =
      video.getAttribute('src') ||
      (video as HTMLVideoElement).src ||
      (video as HTMLSourceElement).src;
    if (src && isValidFileUrl(src)) {
      fileUrls.add(src);
    }
  });

  doc.querySelectorAll('iframe').forEach((iframe) => {
    const src = iframe.src;
    if (src && isValidFileUrl(src)) {
      fileUrls.add(src);
    }
  });

  doc.querySelectorAll('audio source, audio').forEach((audio) => {
    const src =
      audio.getAttribute('src') ||
      (audio as HTMLAudioElement).src ||
      (audio as HTMLSourceElement).src;
    if (src && isValidFileUrl(src)) {
      fileUrls.add(src);
    }
  });

  doc.querySelectorAll('a').forEach((link) => {
    const href = link.href;
    if (href && isValidFileUrl(href) && looksLikeFile(href)) {
      fileUrls.add(href);
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

function looksLikeFile(url: string): boolean {
  const fileExtensions = [
    'jpg',
    'jpeg',
    'png',
    'gif',
    'webp',
    'svg',
    'mp4',
    'mov',
    'avi',
    'mkv',
    'webm',
    'mp3',
    'wav',
    'ogg',
    'pdf',
    'doc',
    'docx',
    'xls',
    'xlsx',
    'ppt',
    'pptx',
    'zip',
    'rar',
    'txt',
  ];

  const urlLower = url.toLowerCase();
  return fileExtensions.some((ext) => urlLower.includes(`.${ext}`));
}

function urlToFileModel(url: string): FileModel {
  const fileName = url.split('/').pop()?.split('?')[0] || 'file';
  const extension = fileName.split('.').pop() || '';

  return {
    name: fileName,
    size: '0',
    ext: toFileExtension(extension),
    url,
  };
}
