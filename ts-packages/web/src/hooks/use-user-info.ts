import { QK_USERS_GET_INFO } from '@/constants';
import { getUserInfo, UserResponse } from '@/lib/api/ratel/me.v3';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export function useUserInfo(): UseQueryResult<UserResponse | null> {
  const query = useQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: async () => {
      try {
        const user = await getUserInfo();
        return user;
      } catch {
        return null;
      }
    },
  });

  return query;
}
