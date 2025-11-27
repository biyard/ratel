import { QK_GET_NOTIFICATIONS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { DeleteNotificationResponse } from '../dto/delete-notification-response';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export function useDeleteNotificationMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (
      notificationId: string,
    ): Promise<DeleteNotificationResponse> => {
      try {
        return await call('DELETE', `/v3/notifications/${notificationId}`);
      } catch (e) {
        logger.error('Failed to delete notification', e);
        throw new Error(String(e));
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_GET_NOTIFICATIONS] });
    },
  });
}
