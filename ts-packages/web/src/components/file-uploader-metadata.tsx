// INFO: return file info
import { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { FileInfo } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { getFileType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { cn } from '@/lib/utils';
import React, { useRef } from 'react';

export interface FileUploaderMetadataProps {
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

    const res: AssetPresignedUris = await get(
      ratelApi.assets.getPresignedUrl(fileTypeKey, totalParts),
    );

    logger.debug('Presigned URL response:', res);

    const { presigned_uris, uris, upload_id, key } = res;

    if (!presigned_uris?.length || !uris?.length || !upload_id || !key) {
      logger.error('Missing presigned upload metadata');
      return;
    }

    const etags: { ETag: string; PartNumber: number }[] = [];

    try {
      for (let partNumber = 0; partNumber < totalParts; partNumber++) {
        const start = partNumber * partSize;
        const end = Math.min(start + partSize, file.size);
        const chunk = file.slice(start, end);
        const url = presigned_uris[partNumber];

        const uploadResp = await fetch(url, {
          method: 'PUT',
          body: chunk,
          credentials: 'same-origin',
        });

        if (!uploadResp.ok) {
          throw new Error(`Upload failed at part ${partNumber + 1}`);
        }

        const etag = uploadResp.headers.get('Etag');
        if (!etag) throw new Error('Missing ETag in part response');

        etags.push({
          ETag: etag.replaceAll('"', ''),
          PartNumber: partNumber + 1,
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
