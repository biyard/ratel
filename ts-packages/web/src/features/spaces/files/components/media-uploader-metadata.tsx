import type { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { getFileType, toContentType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { cn } from '@/lib/utils';
import { useRef } from 'react';
import FileModel from '../types/file';
import {
  completeMultipartUpload,
  getPutMultiObjectUrl,
  getPutObjectUrl,
} from '@/lib/api/ratel/assets.v3';

interface MediaUploaderMetadataProps {
  onUploadSuccess?: (file: FileModel) => void;
  onUploadingChange?: (loading: boolean) => void;
  disabled?: boolean;
}

export default function MediaUploaderMetadata({
  children,
  onUploadSuccess,
  onUploadingChange,
  disabled = false,
  ...props
}: React.ComponentProps<'div'> & MediaUploaderMetadataProps) {
  const inputRef = useRef<HTMLInputElement | null>(null);

  const handleUpload = async () => {
    if (disabled) return;
    inputRef.current?.click();
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      logger.debug('No file selected');
      return;
    }

    const MAX_SIZE = 50 * 1024 * 1024; // 50MB
    if (file.size > MAX_SIZE) {
      showErrorToast('File size exceeds 50MB limit.');
      if (inputRef.current) inputRef.current.value = '';
      return;
    }

    const fileType = file.type;
    const fileTypeKey = getFileType(file);
    logger.debug('FileType:', fileTypeKey);

    const isValidFile = fileType.startsWith('video/');
    if (!isValidFile) {
      showErrorToast('Unsupported file type selected.');
      if (inputRef.current) inputRef.current.value = '';
      return;
    }

    onUploadingChange?.(true);
    const partSize = 5 * 1024 * 1024;
    const totalParts = Math.ceil(file.size / partSize);

    try {
      if (totalParts === 1) {
        const res = await getPutObjectUrl(totalParts, fileTypeKey);
        const presignedUrl = res.presigned_uris[0];
        const publicUrl = res.uris[0];

        const uploadResponse = await fetch(presignedUrl, {
          method: 'PUT',
          headers: { 'Content-Type': toContentType(fileTypeKey) },
          body: file,
        });
        if (!uploadResponse.ok) throw new Error('File upload failed');

        logger.debug('File uploaded successfully:', file.name);
        onUploadSuccess?.({
          id: crypto.randomUUID(),
          name: file.name,
          size: `${(file.size / 1024).toFixed(1)} KB`,
          ext: fileTypeKey,
          url: publicUrl,
        });
      } else {
        const res: AssetPresignedUris = await getPutMultiObjectUrl(
          totalParts,
          fileTypeKey,
        );
        logger.debug('Presigned URL response:', res);

        const { presigned_uris, uris, upload_id, key } = res;
        if (!presigned_uris?.length || !uris?.length || !upload_id || !key) {
          logger.error('Missing presigned upload metadata');
          throw new Error('Missing presigned upload metadata');
        }

        const etags: { etag: string; part_number: number }[] = [];
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
          etags.push({ etag, part_number: realPart });
        }

        await completeMultipartUpload({ upload_id, key, parts: etags });
        logger.debug('Multipart upload completed successfully.');
        onUploadSuccess?.({
          id: crypto.randomUUID(),
          name: file.name,
          size: `${(file.size / 1024).toFixed(1)} KB`,
          ext: fileTypeKey,
          url: uris[0],
        });
      }
    } catch (error) {
      logger.error('Upload error:', error);
      showErrorToast('Failed to upload file. Please try again.');
    } finally {
      if (inputRef.current) inputRef.current.value = '';
      onUploadingChange?.(false);
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
      if (!resp.ok) throw new Error(`Upload failed at part ${partNumber + 1}`);

      let etag: string | null = null;
      for (const [key, value] of resp.headers.entries()) {
        if (key.toLowerCase() === 'etag') {
          etag = value;
          break;
        }
      }
      if (etag) {
        return { etag: etag.replaceAll('"', ''), partNumber: partNumber + 1 };
      }
      console.warn(
        `Retrying upload for part ${partNumber + 1}, attempt ${attempt}`,
      );
    }
    throw new Error(`Missing ETag for part ${partNumber + 1}`);
  };

  const accept = 'video/*';

  return (
    <div
      onClick={handleUpload}
      aria-disabled={disabled}
      className={cn(
        disabled ? 'pointer-events-none select-none' : 'cursor-pointer',
        props.className,
      )}
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
