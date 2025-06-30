import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { useApiCall } from '../use-send';
import { QK_ASSETS_GET_PRESIGNED_URL } from '@/constants';
import { ratelApi } from '../ratel_api';
import { FileType } from '../models/file-type';
import { AssetPresignedUris } from '../models/asset-presigned-uris';

export function usePresignedUrl(
  file_type: FileType,
): UseQueryResult<AssetPresignedUris | undefined> {
  const { get } = useApiCall();

  return useQuery({
    queryKey: [QK_ASSETS_GET_PRESIGNED_URL],
    queryFn: () => get(ratelApi.assets.getPresignedUrl(file_type)),
    refetchOnWindowFocus: false,
  });
}
