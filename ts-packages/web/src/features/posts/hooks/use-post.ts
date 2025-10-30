import {
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { type PostDetailResponse } from '../dto/post-detail-response';
import { call } from '@/lib/api/ratel/call';

export function getPost(postPk: string): Promise<PostDetailResponse> {
  return call('GET', `/v3/posts/${encodeURIComponent(postPk)}`);
}

export function getOption(postPk: string) {
  return {
    queryKey: feedKeys.detail(postPk),
    queryFn: async () => {
      const post = await getPost(postPk);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSuspensePostById(
  postPk: string,
): UseSuspenseQueryResult<PostDetailResponse> {
  const query = useSuspenseQuery({ ...getOption(postPk) });
  return query;
}

export function usePostById(
  postPk?: string,
): UseQueryResult<PostDetailResponse> {
  return useQuery({
    ...getOption(postPk!),
    enabled: !!postPk,
  });
}
