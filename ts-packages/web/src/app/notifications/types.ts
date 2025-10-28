export const NotificationType = {
  UNKNOWN: 0,
  INVITE_TEAM: 1,
  INVITE_DISCUSSION: 2,
  BOOSTING_SPACE: 3,
  CONNECT_NETWORK: 4,
} as const;

export type NotificationType =
  (typeof NotificationType)[keyof typeof NotificationType];

// Frontend-only filter type; compose with the enum instead of polluting it
export type NotificationsFilter = NotificationType | 'all';

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
  notification_type: NotificationType;
  read: boolean;
}

// Special data types for different notification types
export interface TeamInviteSpecialData {
  team_id: number;
  group_id: number;
}

export interface DiscussionInviteSpecialData {
  discussion_id: number;
}

export interface SpaceBoostSpecialData {
  space_id: number;
}

export interface NetworkConnectSpecialData {
  requester_id: number;
}

export type NotificationSpecialData =
  | TeamInviteSpecialData
  | DiscussionInviteSpecialData
  | SpaceBoostSpecialData
  | NetworkConnectSpecialData;

// Helper functions to extract notification information
export function getNotificationType(
  notification: Notification,
): NotificationType {
  return notification.notification_type;
}

export function getNotificationContent(notification: Notification): {
  title: string;
  description: string;
  imageUrl?: string;
  specialData?: NotificationSpecialData;
} | null {
  const { metadata, notification_type } = notification;

  switch (notification_type) {
    case NotificationType.INVITE_TEAM:
      if (metadata.InviteTeam) {
        return {
          title: 'Team Invitation',
          description: metadata.InviteTeam.description,
          imageUrl: metadata.InviteTeam.image_url,
          specialData: {
            team_id: metadata.InviteTeam.team_id,
            group_id: metadata.InviteTeam.group_id,
          },
        };
      }
      break;

    case NotificationType.INVITE_DISCUSSION:
      if (metadata.InviteDiscussion) {
        return {
          title: 'Discussion Invitation',
          description: metadata.InviteDiscussion.description,
          imageUrl: metadata.InviteDiscussion.image_url,
          specialData: {
            discussion_id: metadata.InviteDiscussion.discussion_id,
          },
        };
      }
      break;

    case NotificationType.BOOSTING_SPACE:
      if (metadata.BoostingSpace) {
        return {
          title: 'Space Boosting',
          description: metadata.BoostingSpace.description,
          imageUrl: metadata.BoostingSpace.image_url,
          specialData: {
            space_id: metadata.BoostingSpace.space_id,
          },
        };
      }
      break;

    case NotificationType.CONNECT_NETWORK:
      if (metadata.ConnectNetwork) {
        return {
          title: 'Network Connection',
          description: metadata.ConnectNetwork.description,
          imageUrl: metadata.ConnectNetwork.image_url,
          specialData: {
            requester_id: metadata.ConnectNetwork.requester_id,
          },
        };
      }
      break;

    default:
      return null;
  }

  return null; // Don't show notifications that can't be parsed
}
