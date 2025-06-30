// INFO: return file info
import { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { FileInfo } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { getFileType, toContentType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { cn } from '@/lib/utils';
import React, { useRef } from 'react';

export interface FileUploaderMetadataProps {
  onUploadSuccess?: (fileInfo: FileInfo) => void;
  isImage?: boolean; // true: image only / false: PDF only
}

export default function FileUploaderMetadata({
  children,
  onUploadSuccess,
  isImage = true,
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

    if (isImage && !file.type.startsWith('image/')) {
      alert('Only image files are supported.');
      return;
    }

    if (!isImage && file.type !== 'application/pdf') {
      alert('Only PDF files are supported.');
      return;
    }

    const fileType = getFileType(file);
    logger.debug('FileType:', fileType);

    const res: AssetPresignedUris = await get(
      ratelApi.assets.getPresignedUrl(fileType),
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
          'Content-Type': toContentType(fileType),
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
          ext: fileType.toUpperCase(),
          url: publicUrl,
        };
        onUploadSuccess(fileInfo);
      }
    } catch (error) {
      logger.error('Error uploading file:', error);
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
        accept={isImage ? 'image/*' : 'application/pdf'}
        className="hidden"
        onChange={handleFileChange}
      />
      {children}
    </div>
  );
}
