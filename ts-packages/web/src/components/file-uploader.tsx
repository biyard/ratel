import { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { getFileType, toContentType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { cn } from '@/lib/utils';
import React, { useRef } from 'react';

export interface FileUploaderProps {
  onUploadSuccess?: (url: string) => void;
}

export default function FileUploader({
  children,
  onUploadSuccess,
  ...props
}: React.ComponentProps<'div'> & FileUploaderProps) {
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

    if (!file.type.startsWith('image/')) {
      alert('Only image types are supported.');
      return;
    }

    const fileType = getFileType(file);
    logger.debug('FileType:', fileType);

    const res: AssetPresignedUris = await get(
      ratelApi.assets.getPresignedUrl(fileType),
    );
    logger.debug('Presigned URL response:', res);
    if (
      !res.presigned_uris ||
      res.presigned_uris.length === 0 ||
      !res.uris ||
      res.uris.length === 0
    ) {
      logger.error('No presigned URL received');
      return;
    }
    const presignedUrl = res.presigned_uris[0];
    logger.debug('Presigned URL:', presignedUrl);

    try {
      const uploadResponse = await fetch(presignedUrl, {
        method: 'PUT',
        headers: {
          'Content-Type': toContentType(fileType),
        },
        body: file,
      });

      if (!uploadResponse.ok) {
        throw new Error('File upload failed');
      }
      logger.debug('File uploaded successfully:', file.name);
      if (onUploadSuccess) {
        onUploadSuccess(res.uris[0]);
      }
    } catch (error) {
      logger.error('Error uploading file:', error);
      return;
    }
  };

  return (
    <div
      onClick={handleUpload}
      className={cn('cursor-pointer', props.className)}
      {...props}
    >
      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        className="hidden"
        onChange={handleFileChange}
      />
      {children}
    </div>
  );
}
