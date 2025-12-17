import { QK_GET_NOTIFICATIONS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { MarkAsReadRequest } from '../dto/mark-as-read-request';
import { MarkAsReadResponse } from '../dto/mark-as-read-response';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export function useMarkAsReadMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (
      request: MarkAsReadRequest,
    ): Promise<MarkAsReadResponse> => {
      try {
        return await call('POST', '/v3/notifications/mark-as-read', request);
      } catch (e) {
        logger.error('Failed to mark notifications as read', e);
        throw new Error(String(e));
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_GET_NOTIFICATIONS] });
    },
  });
}
