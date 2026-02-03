interface FileModel {
  id: string;
  name: string;
  size: string;
  ext: FileExtension;
  url?: string | null;
}
export default FileModel;

export enum FileExtension {
  None = 'none',
  PNG = 'png',
  JPG = 'jpg',
  GIF = 'gif',
  WEBM = 'webm',
  SVG = 'svg',
  AI = 'ai',
  ZIP = 'zip',
  PDF = 'pdf',
  XLSX = 'xlsx',

  // 3D Model
  GLB = 'glb',
  GLTF = 'gltf',
  WORD = 'word',
  EXCEL = 'excel',
  // Audio
  MP3 = 'mp3',
  WAV = 'wav',

  // Video
  MP4 = 'mp4',
  MOV = 'mov',
  MKV = 'mkv',

  // Etc
  PPTX = 'pptx',
}

export function toFileExtension(
  input: string | undefined | null,
): FileExtension {
  if (!input) return FileExtension.PDF;

  const s = input.trim().toLowerCase();

  const mimeMap: Record<string, string> = {
    'image/jpeg': 'jpg',
    'image/jpg': 'jpg',
    'image/png': 'png',
    'application/pdf': 'pdf',
    'application/zip': 'zip',
    'application/x-zip-compressed': 'zip',
    'application/msword': 'doc',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document':
      'docx',
    'application/vnd.ms-powerpoint': 'ppt',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation':
      'pptx',
    'application/vnd.ms-excel': 'xls',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'xlsx',
    'text/csv': 'csv',
    'video/mp4': 'mp4',
    'video/mkv': 'mkv',
    'video/quicktime': 'mov',
  };

  let ext = mimeMap[s];

  if (!ext) {
    if (s.includes('.')) {
      ext = s.split('.').pop() || '';
    } else if (s.startsWith('.')) {
      ext = s.slice(1);
    } else {
      ext = s;
    }
  }

  switch (ext) {
    case 'jpg':
    case 'jpeg':
      return FileExtension.JPG;
    case 'png':
      return FileExtension.PNG;
    case 'pdf':
      return FileExtension.PDF;
    case 'zip':
      return FileExtension.ZIP;
    case 'doc':
    case 'docx':
      return FileExtension.WORD;
    case 'ppt':
    case 'pptx':
      return FileExtension.PPTX;
    case 'xls':
    case 'xlsx':
    case 'csv':
      return FileExtension.EXCEL;
    case 'mp4':
      return FileExtension.MP4;
    case 'mkv':
      return FileExtension.MKV;
    case 'mov':
      return FileExtension.MOV;
    default:
      return FileExtension.PDF; // 모르면 PDF로
  }
}

export type BackendFile = Omit<FileModel, 'ext'> & { ext: string };

export const toBackendFile = (f: FileModel): BackendFile => ({
  ...f,
  ext: f.ext.toUpperCase(),
});
