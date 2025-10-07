export const FileType = {
  // Image
  None: 'none',
  PNG: 'png',
  JPG: 'jpg',
  GIF: 'gif',
  WEBM: 'webm',
  SVG: 'svg',
  AI: 'ai',

  PDF: 'pdf',
  XLSX: 'xlsx',

  // 3D Model
  GLB: 'glb',
  GLTF: 'gltf',

  // Audio
  MP3: 'mp3',
  WAV: 'wav',

  // Video
  MP4: 'mp4',
  MOV: 'mov',

  // Etc
  PPTX: 'pptx',
} as const;

export type FileType = typeof FileType[keyof typeof FileType];
