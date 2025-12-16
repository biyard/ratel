import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { parseFileType } from '@/lib/file-utils';
import FileModel from '../types/file';

export async function uploadBase64ImagesToS3(
  files: FileModel[],
): Promise<FileModel[]> {
  const uploadedFiles: FileModel[] = [];

  for (const file of files) {
    if (!file.url || !file.url.startsWith('data:image/')) {
      uploadedFiles.push(file);
      continue;
    }

    try {
      const match = file.url.match(/^data:image\/(\w+);base64,(.+)$/);
      if (!match) {
        continue;
      }

      const [, format] = match;
      const mimeType = `image/${format}`;

      const res = await getPutObjectUrl(1, parseFileType(mimeType));

      if (!res?.presigned_uris?.[0] || !res?.uris?.[0]) {
        continue;
      }

      const blob = await fetch(file.url).then((r) => r.blob());

      const uploadResponse = await fetch(res.presigned_uris[0], {
        method: 'PUT',
        headers: { 'Content-Type': mimeType },
        body: blob,
      });

      if (!uploadResponse.ok) {
        continue;
      }

      uploadedFiles.push({
        ...file,
        url: res.uris[0],
      });
    } catch (error) {
      console.error('Failed to upload base64 image:', error);
    }
  }

  return uploadedFiles;
}
