// INFO: return file info
import { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { FileInfo } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { getFileType, toContentType } from '@/lib/file-utils';
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
  const { get } = useApiCall();

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

    const res: AssetPresignedUris = await get(
      ratelApi.assets.getPresignedUrl(fileTypeKey),
    );
    logger.debug('Presigned URL response:', res);

    if (!res.presigned_uris?.length || !res.uris?.length) {
      logger.error('No presigned URL received');
      return;
    }

    const presignedUrl = res.presigned_uris[0];
    const publicUrl = res.uris[0];

    try {
      const uploadResponse = await fetch(presignedUrl, {
        method: 'PUT',
        headers: {
          'Content-Type': toContentType(fileTypeKey),
        },
        body: file,
      });

      if (!uploadResponse.ok) throw new Error('File upload failed');
      logger.debug('File uploaded successfully:', file.name);

      onUploadSuccess?.({
        name: file.name,
        size: `${(file.size / 1024).toFixed(1)} KB`,
        ext: fileTypeKey.toUpperCase(),
        url: publicUrl,
      });
    } catch (error) {
      logger.error('Error uploading file:', error);
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
