import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { getFileType, toContentType } from '@/lib/file-utils';
import { base64ToFile } from '@/components/text-editor/image-utils';
import { logger } from '@/lib/logger';

/**
 * Hook to upload a base64 image to S3
 * @returns A function that takes a base64 string and optional filename, returns the S3 URL
 */
export function useUploadBase64Image() {
  return async (base64: string, filename?: string): Promise<string> => {
    try {
      // Use provided filename or generate a default one
      const finalFilename = filename || `image-${Date.now()}.png`;

      // Convert base64 to File object
      const file = base64ToFile(base64, finalFilename);

      const fileType = getFileType(file);
      logger.debug('Uploading base64 image:', {
        filename: finalFilename,
        fileType,
      });

      // Get presigned URL
      const res = await getPutObjectUrl(1, fileType);
      logger.debug('Presigned URL response:', res);

      if (
        !res.presigned_uris ||
        res.presigned_uris.length === 0 ||
        !res.uris ||
        res.uris.length === 0
      ) {
        throw new Error('No presigned URL received');
      }

      const presignedUrl = res.presigned_uris[0];
      const s3Url = res.uris[0];

      // Upload to S3
      const uploadResponse = await fetch(presignedUrl, {
        method: 'PUT',
        headers: {
          'Content-Type': toContentType(fileType),
        },
        body: file,
      });

      if (!uploadResponse.ok) {
        throw new Error(
          `File upload failed with status ${uploadResponse.status}`,
        );
      }

      logger.debug('Base64 image uploaded successfully:', {
        filename: finalFilename,
        url: s3Url,
      });
      return s3Url;
    } catch (error) {
      logger.error('Error uploading base64 image:', error);
      throw error;
    }
  };
}
