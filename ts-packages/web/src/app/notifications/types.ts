import { useQuery } from '@tanstack/react-query';
import { useApiCall } from '@/lib/api/use-send';

export enum NotificationType {
  ALL = 'all',
  INVITE_TEAM = 'InviteTeam',
  INVITE_DISCUSSION = 'InviteDiscussion',
  BOOSTING_SPACE = 'BoostingSpace',
  CONNECT_NETWORK = 'ConnectNetwork',
}

// Match the Rust NotificationData structure
export interface NotificationData {
  None?: Record<string, never>;
  InviteTeam?: {
    team_id: number;
    group_id: number;
    image_url?: string;
    description: string;
  };
  InviteDiscussion?: {
    discussion_id: number;
    image_url?: string;
    description: string;
  };
  BoostingSpace?: {
    space_id: number;
    image_url?: string;
    description: string;
  };
  ConnectNetwork?: {
    requester_id: number;
    image_url: string;
    description: string;
  };
}

export interface Notification {
  id: number;
  created_at: number;
  updated_at: number;
  user_id: number;
  metadata: NotificationData;
  read: boolean;
}

// Helper functions to extract notification information
export function getNotificationType(
  metadata: NotificationData,
): NotificationType {
  if (metadata.InviteTeam) return NotificationType.INVITE_TEAM;
  if (metadata.InviteDiscussion) return NotificationType.INVITE_DISCUSSION;
  if (metadata.BoostingSpace) return NotificationType.BOOSTING_SPACE;
  if (metadata.ConnectNetwork) return NotificationType.CONNECT_NETWORK;
  return NotificationType.ALL;
}

export function getNotificationContent(metadata: NotificationData): {
  title: string;
  description: string;
  imageUrl?: string;
} | null {
  if (metadata.InviteTeam) {
    return {
      title: 'Team Invitation',
      description: metadata.InviteTeam.description,
      imageUrl: metadata.InviteTeam.image_url,
    };
  }
  if (metadata.InviteDiscussion) {
    return {
      title: 'Discussion Invitation',
      description: metadata.InviteDiscussion.description,
      imageUrl: metadata.InviteDiscussion.image_url,
    };
  }
  if (metadata.BoostingSpace) {
    return {
      title: 'Space Boosting',
      description: metadata.BoostingSpace.description,
      imageUrl: metadata.BoostingSpace.image_url,
    };
  }
  if (metadata.ConnectNetwork) {
    return {
      title: 'Network Connection',
      description: metadata.ConnectNetwork.description,
      imageUrl: metadata.ConnectNetwork.image_url,
    };
  }
  return null; // Don't show notifications that can't be parsed
}

// Hook for fetching notifications with polling
export function useNotifications(
  filterType: NotificationType = NotificationType.ALL,
) {
  const { get } = useApiCall();

  return useQuery({
    queryKey: ['notifications'],
    queryFn: async () => {
      const response = await get(
        '/v1/notifications?param-type=query&limit=100&size=100',
      );
      // The API returns a QueryResponse<NotificationSummary> structure
      return response?.items || [];
    },
    select: (allNotifications: Notification[]) => {
      // Filter notifications on the frontend
      if (filterType === NotificationType.ALL) {
        return allNotifications;
      }

      return allNotifications.filter((notification: Notification) => {
        const notificationType = getNotificationType(notification.metadata);
        return notificationType === filterType;
      });
    },
    refetchInterval: 5000, // Poll every 5 seconds
    refetchOnWindowFocus: true,
  });
}
