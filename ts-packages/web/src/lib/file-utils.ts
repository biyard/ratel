// import { FileType } from './api/models/file-type';

import { FileExtension } from '@/features/spaces/files/types/file';

export function getFileType(file: File): FileExtension {
  const mime = file.type;
  const name = file.name.toLowerCase();

  if (mime === 'image/png' || name.endsWith('.png')) return FileExtension.PNG;
  if (mime === 'image/jpeg' || name.endsWith('.jpg') || name.endsWith('.jpeg'))
    return FileExtension.JPG;
  if (mime === 'image/gif' || name.endsWith('.gif')) return FileExtension.GIF;
  if (mime === 'image/webp' || name.endsWith('.webm'))
    return FileExtension.WEBM;
  if (mime === 'image/svg+xml' || name.endsWith('.svg'))
    return FileExtension.SVG;
  if (name.endsWith('.ai')) return FileExtension.AI;

  if (mime === 'application/pdf' || name.endsWith('.pdf'))
    return FileExtension.PDF;
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' ||
    name.endsWith('.xlsx')
  )
    return FileExtension.XLSX;
  if (mime === 'application/vnd.ms-excel' || name.endsWith('.xls'))
    return FileExtension.EXCEL;
  if (mime === 'application/msword' || name.endsWith('.doc'))
    return FileExtension.WORD;
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document' ||
    name.endsWith('.docx')
  )
    return FileExtension.WORD;

  if (name.endsWith('.glb')) return FileExtension.GLB;
  if (name.endsWith('.gltf')) return FileExtension.GLTF;

  if (mime === 'audio/mpeg' || name.endsWith('.mp3')) return FileExtension.MP3;
  if (mime === 'audio/wav' || name.endsWith('.wav')) return FileExtension.WAV;

  if (mime === 'video/mp4' || name.endsWith('.mp4')) return FileExtension.MP4;
  if (mime === 'video/mov' || name.endsWith('.mov')) return FileExtension.MOV;

  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.presentationml.presentation' ||
    name.endsWith('.pptx')
  )
    return FileExtension.PPTX;

  return FileExtension.None;
}

export function toContentType(ext: FileExtension): string {
  // Note: For WORD and EXCEL enums, this returns the modern format MIME types (DOCX, XLSX).
  // When uploading files, prefer using the original file.type to preserve the exact MIME type.
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
    case FileExtension.XLSX:
      return 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
    case FileExtension.EXCEL:
      return 'application/vnd.ms-excel';
    case FileExtension.WORD:
      return 'application/vnd.openxmlformats-officedocument.wordprocessingml.document';
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
      return 'video/mov';
    case FileExtension.PPTX:
      return 'application/vnd.openxmlformats-officedocument.presentationml.presentation';
    default:
      return '';
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
    case 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet':
      return FileExtension.XLSX;
    case 'application/vnd.ms-excel':
      return FileExtension.EXCEL;
    case 'application/msword':
      return FileExtension.WORD;
    case 'application/vnd.openxmlformats-officedocument.wordprocessingml.document':
      return FileExtension.WORD;
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
    case 'video/mov':
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
