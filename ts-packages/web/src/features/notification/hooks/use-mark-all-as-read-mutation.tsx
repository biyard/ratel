import { QK_GET_NOTIFICATIONS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { MarkAsReadResponse } from '../dto/mark-as-read-response';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export function useMarkAllAsReadMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (): Promise<MarkAsReadResponse> => {
      try {
        return await call('POST', '/v3/notifications/mark-all-as-read');
      } catch (e) {
        logger.error('Failed to mark all notifications as read', e);
        throw new Error(String(e));
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_GET_NOTIFICATIONS] });
    },
  });
}
