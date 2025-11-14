import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';
import { SpacePostCommentResponse } from '../types/space-post-comment-response';
import { ListResponse } from '@/lib/api/ratel/common';

export const getSpaceComments = async (
  spacePk: string,
  postPk: string,
  bookmark?: string,
): Promise<ListResponse<SpacePostCommentResponse>> => {
  const qs = bookmark ? `?bookmark=${encodeURIComponent(bookmark)}` : '';
  const response = await call(
    'GET',
    `/v3/spaces/${encodeURIComponent(
      spacePk,
    )}/boards/${encodeURIComponent(postPk)}/comments${qs}`,
  );
  return response as ListResponse<SpacePostCommentResponse>;
};

export function getCommentsOption(spacePk: string, postPk: string) {
  return {
    queryKey: spaceKeys.boards_comments(spacePk, postPk),
    queryFn: async () => {
      const res = await getSpaceComments(spacePk, postPk);
      return res;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceComments(
  spacePk: string,
  postPk: string,
): UseQueryResult<ListResponse<SpacePostCommentResponse>> {
  const query = useQuery({
    ...getCommentsOption(spacePk, postPk),
    enabled: !!spacePk && !!postPk,
  });
  return query;
}
