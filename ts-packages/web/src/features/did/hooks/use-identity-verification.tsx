import { feedKeys } from '@/constants';
import { comment } from '@/lib/api/ratel/comments.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { PostDetailResponse } from '../dto/post-detail-response';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import * as PortOne from '@portone/browser-sdk/v2';
import { config } from '@/config';
import { call } from '@/lib/api/ratel/call';
import { useState } from 'react';
import { logger } from '@/lib/logger';

export function useIdentityVerification() {
  const { data: user } = useSuspenseUserInfo();
  const [verifying, setVerifying] = useState(false);

  const mutation = useMutation({
    mutationFn: async () => {
      const identityVerificationId = `iv-${user.pk}-${crypto.randomUUID()}`;
      setVerifying(true);
      const response = await PortOne.requestIdentityVerification({
        storeId: config.portone_store_id,
        identityVerificationId,
        channelKey: config.portone_inicis_channel_key,
      });
      setVerifying(false);
      logger.debug('identity verification response', response);
      if (response.code !== undefined) {
        return alert(response.message);
      }

      const resp = await call('POST', '/v3/did/kyc', {
        id: identityVerificationId,
      });

      return { resp };
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { mutation, verifying };
}
