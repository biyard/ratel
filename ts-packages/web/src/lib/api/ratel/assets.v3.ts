import { FileExtension } from '@/features/spaces/files/types/file';
import { call } from './call';
import { AssetPresignedUris } from '../models/asset-presigned-uris';

export function getPutObjectUrl(
  total_count: number | null | undefined,
  file_type: FileExtension,
): Promise<AssetPresignedUris> {
  return call(
    'GET',
    `/v3/assets?total_count=${total_count}&file_type=${file_type}`,
  );
}

export function getPutMultiObjectUrl(
  total_count: number | null | undefined,
  file_type: FileExtension,
): Promise<AssetPresignedUris> {
  return call(
    'GET',
    `/v3/assets/multiparts?total_count=${total_count}&file_type=${file_type}`,
  );
}

export interface CompleteMultipartUploadBody {
  upload_id: string;
  key: string;
  parts: {
    part_number: number;
    etag: string;
  }[];
}
export function completeMultipartUpload(
  body: CompleteMultipartUploadBody,
): Promise<void> {
  return call('POST', `/v3/assets/multiparts`, body);
}
