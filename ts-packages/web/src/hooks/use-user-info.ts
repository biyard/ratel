import { QK_USERS_GET_INFO } from '@/constants';
import { getMe, User } from '@/lib/api/ratel/auth.v3';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export function useUserInfo(): UseQueryResult<User | undefined> {
  const query = useQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: () => getMe(),
  });

  return query;
}
