import type { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { getFileType, toContentType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { cn } from '@/lib/utils';
import React, { useRef, forwardRef, useImperativeHandle } from 'react';
import {
  completeMultipartUpload,
  getPutMultiObjectUrl,
  getPutObjectUrl,
} from '@/lib/api/ratel/assets.v3';
import FileModel, { FileExtension } from '../types/file';

export type FileUploaderHandle = {
  uploadFile: (file: File) => Promise<void>;
  openPicker: () => void;
};

interface FileUploaderMetadataProps extends React.ComponentProps<'div'> {
  onUploadSuccess?: (file: FileModel) => void;
  onUploadingChange?: (loading: boolean) => void;
  disabled?: boolean;
  isImage?: boolean;
  isMedia?: boolean;
  maxSizeMB?: number;
}

const FileUploaderMetadata = forwardRef<
  FileUploaderHandle,
  FileUploaderMetadataProps
>(
  (
    {
      children,
      onUploadSuccess,
      onUploadingChange,
      disabled = false,
      className,
      isImage,
      isMedia,
      maxSizeMB = 50,
      ...props
    },
    ref,
  ) => {
    const inputRef = useRef<HTMLInputElement | null>(null);
    const openingRef = useRef(false);

    const openPicker = () => {
      if (disabled || openingRef.current) return;
      openingRef.current = true;
      inputRef.current?.click();
      setTimeout(() => {
        openingRef.current = false;
      }, 0);
    };

    const validateFile = (file: File) => {
      const maxBytes = maxSizeMB * 1024 * 1024;
      if (file.size > maxBytes) {
        showErrorToast(`File size exceeds ${maxSizeMB}MB limit.`);
        return false;
      }
      if (isImage && !file.type.startsWith('image/')) {
        showErrorToast('Only image files are allowed.');
        return false;
      }
      if (isMedia && !file.type.startsWith('video/')) {
        showErrorToast('Only video files are allowed.');
        return false;
      }
      return true;
    };

    const uploadPdf = async (file: File) => {
      try {
        if (file.type !== 'application/pdf') {
          showErrorToast('Only PDF files are allowed.');
          return;
        }

        const res = await getPutObjectUrl(1, FileExtension.PDF);
        const presignedUrl = res.presigned_uris?.[0];
        const publicUrl = res.uris?.[0];
        if (!presignedUrl || !publicUrl) {
          showErrorToast('Failed to issue upload URL.');
          return;
        }

        const uploadResponse = await fetch(presignedUrl, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/pdf' },
          body: file,
        });
        if (!uploadResponse.ok) throw new Error('PDF upload failed');

        onUploadSuccess?.({
          id: crypto.randomUUID(),
          name: file.name,
          size: `${(file.size / 1024 / 1024).toFixed(2)} MB`,
          ext: FileExtension.PDF,
          url: publicUrl,
        });
      } catch (error) {
        logger.error('PDF upload failed:', error);
        showErrorToast('Failed to upload PDF. Please try again.');
      }
    };

    const uploadNonPdf = async (file: File, fileTypeKey: FileExtension) => {
      const partSize = 5 * 1024 * 1024;
      const totalParts = Math.ceil(file.size / partSize);

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
        onUploadSuccess?.({
          id: crypto.randomUUID(),
          name: file.name,
          size: `${(file.size / 1024).toFixed(1)} KB`,
          ext: fileTypeKey,
          url: publicUrl,
        });
        return;
      }

      const res: AssetPresignedUris = await getPutMultiObjectUrl(
        totalParts,
        fileTypeKey,
      );
      const { presigned_uris, uris, upload_id, key } = res;
      if (!presigned_uris?.length || !uris?.length || !upload_id || !key) {
        throw new Error('Missing presigned upload metadata');
      }

      const etags: { etag: string; part_number: number }[] = [];
      for (let partNumber = 0; partNumber < totalParts; partNumber++) {
        const start = partNumber * partSize;
        const end = Math.min(start + partSize, file.size);
        const chunk = file.slice(start, end);
        const url = presigned_uris[partNumber];
        const resp = await fetch(url, {
          method: 'PUT',
          body: chunk,
          credentials: 'omit',
        });
        if (!resp.ok)
          throw new Error(`Upload failed at part ${partNumber + 1}`);
        const etagHeader = resp.headers.get('etag');
        if (!etagHeader)
          throw new Error(`Missing ETag for part ${partNumber + 1}`);
        etags.push({
          etag: etagHeader.replaceAll('"', ''),
          part_number: partNumber + 1,
        });
      }

      await completeMultipartUpload({ upload_id, key, parts: etags });
      onUploadSuccess?.({
        id: crypto.randomUUID(),
        name: file.name,
        size: `${(file.size / 1024).toFixed(1)} KB`,
        ext: fileTypeKey,
        url: uris[0],
      });
    };

    const uploadFile = async (file: File) => {
      if (!validateFile(file)) return;

      let fileTypeKey = getFileType(file);
      onUploadingChange?.(true);

      if (fileTypeKey === 'none') {
        fileTypeKey = FileExtension.MKV;
      }

      try {
        if (
          file.type === 'application/pdf' ||
          fileTypeKey === FileExtension.PDF
        ) {
          await uploadPdf(file);
        } else {
          await uploadNonPdf(file, fileTypeKey);
        }
      } catch (error) {
        logger.error('Upload error:', error);
        showErrorToast('Failed to upload file. Please try again.');
      } finally {
        if (inputRef.current) inputRef.current.value = '';
        onUploadingChange?.(false);
      }
    };

    useImperativeHandle(ref, () => ({ uploadFile, openPicker }));

    const handleChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;
      await uploadFile(file);
    };

    const accept = isImage ? 'image/*' : isMedia ? 'video/*' : '*/*';

    return (
      <div
        aria-disabled={disabled}
        className={cn(
          disabled ? 'pointer-events-none select-none' : '',
          className,
        )}
        {...props}
      >
        <input
          ref={inputRef}
          type="file"
          accept={accept}
          className="hidden"
          onChange={handleChange}
        />
        {children}
      </div>
    );
  },
);

export default FileUploaderMetadata;
