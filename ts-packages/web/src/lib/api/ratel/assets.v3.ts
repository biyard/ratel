import { FileType } from '../models/file-type';
import { call } from './call';

export function getPutObjectUrl(
  total_count: number | null | undefined,
  file_type: FileType,
): Promise<void> {
  return call(
    'GET',
    `/v3/assets?total_count=${total_count}&file_type=${file_type}`,
  );
}

export function getPutMultiObjectUrl(
  total_count: number | null | undefined,
  file_type: FileType,
): Promise<void> {
  return call(
    'GET',
    `/v3/assets/multiparts?total_count=${total_count}&file_type=${file_type}`,
  );
}

export function completeMultipartUpload(
  total_count: number | null | undefined,
  file_type: FileType,
): Promise<void> {
  return call(
    'GET',
    `/v3/assets/multiparts/complete?total_count=${total_count}&file_type=${file_type}`,
  );
}
