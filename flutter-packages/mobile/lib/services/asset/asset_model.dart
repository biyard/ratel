class AssetPresignedUris {
  final List<String> presignedUris;
  final List<String> uris;

  AssetPresignedUris({required this.presignedUris, required this.uris});

  factory AssetPresignedUris.fromJson(Map<String, dynamic> json) {
    final p =
        (json['presigned_uris'] as List?)?.map((e) => e.toString()).toList() ??
        const [];
    final u =
        (json['uris'] as List?)?.map((e) => e.toString()).toList() ?? const [];
    return AssetPresignedUris(presignedUris: p, uris: u);
  }
}

enum FileType {
  // Image
  none,
  png,
  jpg,
  gif,
  webm,
  svg,
  ai,

  // Document
  pdf,
  xlsx,
  pptx,

  // 3D Model
  glb,
  gltf,

  // Audio
  mp3,
  wav,

  // Video
  mp4,
  mov,
}

extension FileTypeExt on FileType {
  String get value {
    switch (this) {
      case FileType.none:
        return 'none';
      case FileType.png:
        return 'png';
      case FileType.jpg:
        return 'jpg';
      case FileType.gif:
        return 'gif';
      case FileType.webm:
        return 'webm';
      case FileType.svg:
        return 'svg';
      case FileType.ai:
        return 'ai';
      case FileType.pdf:
        return 'pdf';
      case FileType.xlsx:
        return 'xlsx';
      case FileType.pptx:
        return 'pptx';
      case FileType.glb:
        return 'glb';
      case FileType.gltf:
        return 'gltf';
      case FileType.mp3:
        return 'mp3';
      case FileType.wav:
        return 'wav';
      case FileType.mp4:
        return 'mp4';
      case FileType.mov:
        return 'mov';
    }
  }

  /// HTTP Content-Type
  String get contentType {
    switch (this) {
      case FileType.png:
        return 'image/png';
      case FileType.jpg:
        return 'image/jpeg';
      case FileType.gif:
        return 'image/gif';
      case FileType.webm:
        return 'image/webp';
      case FileType.svg:
        return 'image/svg+xml';
      case FileType.ai:
        return 'application/postscript';
      case FileType.pdf:
        return 'application/pdf';
      case FileType.xlsx:
        return 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
      case FileType.pptx:
        return 'application/vnd.openxmlformats-officedocument.presentationml.presentation';
      case FileType.glb:
        return 'model/gltf-binary';
      case FileType.gltf:
        return 'model/gltf+json';
      case FileType.mp3:
        return 'audio/mpeg';
      case FileType.wav:
        return 'audio/wav';
      case FileType.mp4:
        return 'video/mp4';
      case FileType.mov:
        return 'video/quicktime';
      case FileType.none:
        return 'application/octet-stream';
    }
  }
}

/// path(혹은 파일명)에서 확장자 추출 → FileType 매핑
FileType fileTypeFromPath(String path) {
  final ext = path.split('.').last.toLowerCase();
  switch (ext) {
    case 'png':
      return FileType.png;
    case 'jpg':
    case 'jpeg':
      return FileType.jpg;
    case 'gif':
      return FileType.gif;
    case 'webp':
    case 'webm':
      return FileType.webm;
    case 'svg':
      return FileType.svg;
    case 'ai':
      return FileType.ai;
    case 'pdf':
      return FileType.pdf;
    case 'xlsx':
      return FileType.xlsx;
    case 'pptx':
      return FileType.pptx;
    case 'glb':
      return FileType.glb;
    case 'gltf':
      return FileType.gltf;
    case 'mp3':
      return FileType.mp3;
    case 'wav':
      return FileType.wav;
    case 'mp4':
      return FileType.mp4;
    case 'mov':
      return FileType.mov;
    default:
      return FileType.none;
  }
}
