import { FileType } from './api/models/file-type';

export function getFileType(file: File): FileType {
  const mime = file.type;
  const name = file.name.toLowerCase();

  if (mime === 'image/png' || name.endsWith('.png')) return FileType.PNG;
  if (mime === 'image/jpeg' || name.endsWith('.jpg') || name.endsWith('.jpeg'))
    return FileType.JPG;
  if (mime === 'image/gif' || name.endsWith('.gif')) return FileType.GIF;
  if (mime === 'image/webp' || name.endsWith('.webm')) return FileType.WEBM;
  if (mime === 'image/svg+xml' || name.endsWith('.svg')) return FileType.SVG;
  if (name.endsWith('.ai')) return FileType.AI;

  if (mime === 'application/pdf' || name.endsWith('.pdf')) return FileType.PDF;
  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' ||
    name.endsWith('.xlsx')
  )
    return FileType.XLSX;

  if (name.endsWith('.glb')) return FileType.GLB;
  if (name.endsWith('.gltf')) return FileType.GLTF;

  if (mime === 'audio/mpeg' || name.endsWith('.mp3')) return FileType.MP3;
  if (mime === 'audio/wav' || name.endsWith('.wav')) return FileType.WAV;

  if (mime === 'video/mp4' || name.endsWith('.mp4')) return FileType.MP4;

  if (
    mime ===
      'application/vnd.openxmlformats-officedocument.presentationml.presentation' ||
    name.endsWith('.pptx')
  )
    return FileType.PPTX;

  return FileType.None;
}

export function toContentType(fileType: FileType): string {
  switch (fileType) {
    case FileType.PNG:
      return 'image/png';
    case FileType.JPG:
      return 'image/jpeg';
    case FileType.GIF:
      return 'image/gif';
    case FileType.WEBM:
      return 'image/webp';
    case FileType.SVG:
      return 'image/svg+xml';
    case FileType.AI:
      return 'application/postscript';
    case FileType.PDF:
      return 'application/pdf';
    case FileType.XLSX:
      return 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
    case FileType.GLB:
      return 'model/gltf-binary';
    case FileType.GLTF:
      return 'model/gltf+json';
    case FileType.MP3:
      return 'audio/mpeg';
    case FileType.WAV:
      return 'audio/wav';
    case FileType.MP4:
      return 'video/mp4';
    case FileType.PPTX:
      return 'application/vnd.openxmlformats-officedocument.presentationml.presentation';
    default:
      return '';
  }
}
