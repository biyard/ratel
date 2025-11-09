import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { VerifiedAttributes } from '../types/verified_attributes';
import { optimisticUpdate } from '@/lib/hook-utils';
import { UserAttributes } from '../types/user-attributes';

export function useCodeVerification() {
  const mutation = useMutation({
    mutationFn: async (code: string) => {
      logger.debug('Verifying code:', code);

      const resp = await call('PUT', '/v3/me/did', {
        type: 'code',
        code,
      });

      return { attributes: new VerifiedAttributes(resp) };
    },
    onSuccess: async ({ attributes }) => {
      optimisticUpdate<UserAttributes>(
        { queryKey: ['user-verified-attributes'] },
        (old) => {
          if (attributes.age) old.age = attributes.age;
          if (attributes.gender) old.gender = attributes.gender;
          if (attributes.university) old.university = attributes.university;

          return old;
        },
      );
    },
  });

  return mutation;
}
