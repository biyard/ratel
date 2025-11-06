import { call } from '@/lib/api/ratel/call';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { UserAttributes } from '../types/user-attributes';

export function useVerifiedAttributes(): UseSuspenseQueryResult<UserAttributes> {
  return useSuspenseQuery({
    queryKey: ['user-verified-attributes'],
    queryFn: async () => {
      const json = await call('GET', '/v3/me/did/attributes');

      return new UserAttributes(json);
    },
  });
}
