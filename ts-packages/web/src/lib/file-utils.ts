// import { FileType } from './api/models/file-type';

import { FileExtension } from '@/features/spaces/files/types/file';

export function getFileType(file: File): FileExtension {
  const mime = file.type;
  const name = file.name.toLowerCase();

  // Image types
  if (mime === 'image/png' || name.endsWith('.png')) return FileExtension.PNG;
  if (mime === 'image/jpeg' || name.endsWith('.jpg') || name.endsWith('.jpeg'))
    return FileExtension.JPG;
  if (mime === 'image/gif' || name.endsWith('.gif')) return FileExtension.GIF;
  if (mime === 'image/webp' || name.endsWith('.webp'))
    return FileExtension.WEBM;
  if (mime === 'image/svg+xml' || name.endsWith('.svg'))
    return FileExtension.SVG;
  if (name.endsWith('.ai')) return FileExtension.AI;

  // Document types
  if (mime === 'application/pdf' || name.endsWith('.pdf'))
    return FileExtension.PDF;
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document' ||
    name.endsWith('.docx')
  )
    return FileExtension.DOCX;
  if (mime === 'application/msword' || name.endsWith('.doc'))
    return FileExtension.DOC;
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' ||
    name.endsWith('.xlsx')
  )
    return FileExtension.XLSX;
  if (mime === 'application/vnd.ms-excel' || name.endsWith('.xls'))
    return FileExtension.XLS;
  if (
    mime === 'application/zip' ||
    mime === 'application/x-zip-compressed' ||
    name.endsWith('.zip')
  )
    return FileExtension.ZIP;

  // 3D Model types
  if (name.endsWith('.glb')) return FileExtension.GLB;
  if (name.endsWith('.gltf')) return FileExtension.GLTF;

  // Audio types
  if (mime === 'audio/mpeg' || name.endsWith('.mp3')) return FileExtension.MP3;
  if (mime === 'audio/wav' || name.endsWith('.wav')) return FileExtension.WAV;

  // Video types
  if (mime === 'video/mp4' || name.endsWith('.mp4')) return FileExtension.MP4;
  if (mime === 'video/quicktime' || name.endsWith('.mov'))
    return FileExtension.MOV;

  // Presentation types
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.presentationml.presentation' ||
    name.endsWith('.pptx')
  )
    return FileExtension.PPTX;

  return FileExtension.None;
}

export function toContentType(ext: FileExtension): string {
  switch (ext) {
    case FileExtension.PNG:
      return 'image/png';
    case FileExtension.JPG:
      return 'image/jpeg';
    case FileExtension.GIF:
      return 'image/gif';
    case FileExtension.WEBM:
      return 'image/webp';
    case FileExtension.SVG:
      return 'image/svg+xml';
    case FileExtension.AI:
      return 'application/postscript';
    case FileExtension.PDF:
      return 'application/pdf';
    case FileExtension.DOCX:
      return 'application/vnd.openxmlformats-officedocument.wordprocessingml.document';
    case FileExtension.DOC:
      return 'application/msword';
    case FileExtension.XLSX:
      return 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
    case FileExtension.XLS:
      return 'application/vnd.ms-excel';
    case FileExtension.ZIP:
      return 'application/zip';
    case FileExtension.GLB:
      return 'model/gltf-binary';
    case FileExtension.GLTF:
      return 'model/gltf+json';
    case FileExtension.MP3:
      return 'audio/mpeg';
    case FileExtension.WAV:
      return 'audio/wav';
    case FileExtension.MP4:
      return 'video/mp4';
    case FileExtension.MOV:
      return 'video/quicktime';
    case FileExtension.PPTX:
      return 'application/vnd.openxmlformats-officedocument.presentationml.presentation';
    default:
      return 'application/octet-stream';
  }
}

export function parseFileType(mime: string): FileExtension {
  switch (mime) {
    case 'image/png':
      return FileExtension.PNG;
    case 'image/jpeg':
      return FileExtension.JPG;
    case 'image/gif':
      return FileExtension.GIF;
    case 'image/webp':
      return FileExtension.WEBM;
    case 'image/svg+xml':
      return FileExtension.SVG;
    case 'application/postscript':
      return FileExtension.AI;
    case 'application/pdf':
      return FileExtension.PDF;
    case 'application/vnd.openxmlformats-officedocument.wordprocessingml.document':
      return FileExtension.DOCX;
    case 'application/msword':
      return FileExtension.DOC;
    case 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet':
      return FileExtension.XLSX;
    case 'application/vnd.ms-excel':
      return FileExtension.XLS;
    case 'application/zip':
    case 'application/x-zip-compressed':
      return FileExtension.ZIP;
    case 'model/gltf-binary':
      return FileExtension.GLB;
    case 'model/gltf+json':
      return FileExtension.GLTF;
    case 'audio/mpeg':
      return FileExtension.MP3;
    case 'audio/wav':
      return FileExtension.WAV;
    case 'video/mp4':
      return FileExtension.MP4;
    case 'video/quicktime':
      return FileExtension.MOV;
    case 'application/vnd.openxmlformats-officedocument.presentationml.presentation':
      return FileExtension.PPTX;
    default:
      return FileExtension.None;
  }
}

export const dataUrlToBlob = async (dataUrl: string): Promise<Blob> => {
  const res = await fetch(dataUrl);
  return await res.blob();
};
