import { FileExtension } from '@/features/spaces/files/types/file';

export interface AssetPresignedUris {
  presigned_uris: string[];
  uris: string[];
  total_count: number;
  file_type: FileExtension;

  upload_id?: string;
  key?: string;
}
