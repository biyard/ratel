// INFO: return file info
import type { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { FileInfo } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { getFileType, toContentType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { cn } from '@/lib/utils';
import { useRef } from 'react';

interface FileUploaderMetadataProps {
  onUploadSuccess?: (fileInfo: FileInfo) => void;
  isImage?: boolean; // true: image only / false: PDF only
  isMedia?: boolean;
}

export default function FileUploaderMetadata({
  children,
  onUploadSuccess,
  isImage = true,
  isMedia = false,
  ...props
}: React.ComponentProps<'div'> & FileUploaderMetadataProps) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const { get, post } = useApiCall();

  const handleUpload = async () => {
    inputRef.current?.click();
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      logger.debug('No file selected');
      return;
    }

    const fileType = file.type;
    const fileName = file.name;

    const isImageFile = fileType.startsWith('image/');
    const isVideoFile = fileType.startsWith('video/');
    const isPdfFile = fileType === 'application/pdf';
    const isZipFile =
      fileType === 'application/zip' || fileName.endsWith('.zip');

    const fileTypeKey = getFileType(file);
    logger.debug('FileType:', fileTypeKey);

    const isValidFile = (() => {
      if (isImage && isMedia) return isImageFile || isVideoFile;
      if (isImage && !isMedia) return isImageFile;
      if (!isImage && isMedia) return isVideoFile || isPdfFile || isZipFile;
      return isPdfFile;
    })();

    if (!isValidFile) {
      showErrorToast('Unsupported file type selected.');
      return;
    }

    const partSize = 5 * 1024 * 1024;
    const totalParts = Math.ceil(file.size / partSize);

    const res = await get(
      ratelApi.assets.getMultipartPresignedUrl(fileTypeKey, totalParts),
    );

    if (totalParts === 1) {
      const res: AssetPresignedUris = await get(
        ratelApi.assets.getPresignedUrl(fileTypeKey, totalParts),
      );

      const presignedUrl = res.presigned_uris[0];
      const publicUrl = res.uris[0];

      const uploadResponse = await fetch(presignedUrl, {
        method: 'PUT',
        headers: {
          'Content-Type': toContentType(fileTypeKey),
        },
        body: file,
      });

      if (!uploadResponse.ok) {
        throw new Error('File upload failed');
      }

      logger.debug('File uploaded successfully:', file.name);

      if (onUploadSuccess) {
        const fileInfo: FileInfo = {
          name: file.name,
          size: `${(file.size / 1024).toFixed(1)} KB`,
          ext: fileTypeKey.toUpperCase(),
          url: publicUrl,
        };

        onUploadSuccess(fileInfo);
      }
    } else {
      const res: AssetPresignedUris = await get(
        ratelApi.assets.getMultipartPresignedUrl(fileTypeKey, totalParts),
      );
      logger.debug('Presigned URL response:', res);

      const { presigned_uris, uris, upload_id, key } = res;

      if (!presigned_uris?.length || !uris?.length || !upload_id || !key) {
        logger.error('Missing presigned upload metadata');
        return;
      }

      const etags: { etag: string; part_number: number }[] = [];

      try {
        for (let partNumber = 0; partNumber < totalParts; partNumber++) {
          const start = partNumber * partSize;
          const end = Math.min(start + partSize, file.size);
          const chunk = file.slice(start, end);
          const url = presigned_uris[partNumber];

          const { etag, partNumber: realPart } = await fetchWithETag(
            url,
            chunk,
            partNumber,
          );

          etags.push({
            etag: etag,
            part_number: realPart,
          });
        }

        await post(ratelApi.assets.createMultipartUpload(), {
          upload_id,
          key,
          parts: etags,
        });

        logger.debug('Multipart upload completed successfully.');

        onUploadSuccess?.({
          name: file.name,
          size: `${(file.size / 1024).toFixed(1)} KB`,
          ext: fileTypeKey.toUpperCase(),
          url: uris[0],
        });
      } catch (error) {
        logger.error('Multipart upload error:', error);
        showErrorToast('Failed to upload file. Please try again.');
      }
    }
  };

  const fetchWithETag = async (
    url: string,
    chunk: Blob,
    partNumber: number,
  ): Promise<{ etag: string; partNumber: number }> => {
    for (let attempt = 1; attempt <= 5; attempt++) {
      const resp = await fetch(url, {
        method: 'PUT',
        body: chunk,
        credentials: 'omit',
      });

      if (!resp.ok) {
        throw new Error(`Upload failed at part ${partNumber + 1}`);
      }

      let etag: string | null = null;
      for (const [key, value] of resp.headers.entries()) {
        if (key.toLowerCase() === 'etag') {
          etag = value;
          break;
        }
      }

      if (etag) {
        return {
          etag: etag.replaceAll('"', ''),
          partNumber: partNumber + 1,
        };
      }

      console.warn(
        `Retrying upload for part ${partNumber + 1}, attempt ${attempt}`,
      );
    }

    throw new Error(`Missing ETag for part ${partNumber + 1}`);
  };

  const accept = (() => {
    if (isImage && isMedia) return 'image/*,video/*';
    if (isImage && !isMedia) return 'image/*';
    if (!isImage && isMedia)
      return 'video/*,application/pdf,application/zip,.zip';
    return 'application/pdf';
  })();

  return (
    <div
      onClick={handleUpload}
      className={cn('cursor-pointer', props.className)}
      {...props}
    >
      <input
        ref={inputRef}
        type="file"
        accept={accept}
        className="hidden"
        onChange={handleFileChange}
      />
      {children}
    </div>
  );
}
