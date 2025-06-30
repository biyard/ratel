import { FileType } from './file-type';

export interface AssetPresignedUris {
  presigned_uris: string[];
  uris: string[];
  total_count: number;
  file_type: FileType;
}
