import { useNotificationsI18n } from '@/features/notification/i18n';
import { useNotificationsPageController } from '@/app/notifications/use-notifications-page-controller';
import { Bell, Delete_2, CheckCircle } from '@/components/icons';
import {
  getNotificationType,
  getNotificationData,
} from '@/features/notification/types/email-operation';
import { formatDistanceToNow } from 'date-fns';

export default function NotificationsPage() {
  const ctrl = useNotificationsPageController();
  const i18n = useNotificationsI18n();

  if (ctrl.isLoading) {
    return (
      <div className="w-full max-w-desktop mx-auto px-4 py-8">
        <div className="text-center text-foreground">{i18n.title}...</div>
      </div>
    );
  }

  if (ctrl.error) {
    return (
      <div className="w-full max-w-desktop mx-auto px-4 py-8">
        <div className="bg-card-bg border border-card-border rounded-lg p-8">
          <div className="text-center text-destructive">
            Error: {ctrl.error.message}
          </div>
        </div>
      </div>
    );
  }

  // Helper to convert notification type to i18n key
  const getNotificationTypeKey = (type: string): keyof typeof i18n => {
    const typeMap: Record<string, keyof typeof i18n> = {
      SpacePostNotification: 'space_post_notification',
      TeamInvite: 'team_invite',
      SpaceInviteVerification: 'space_invite_verification',
      SignupSecurityCode: 'signup_security_code',
      StartSurvey: 'start_survey',
    };
    return (typeMap[type] || 'unknown') as keyof typeof i18n;
  };

  return (
    <div className="w-full max-w-desktop mx-auto px-4 py-6">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-xl bg-primary/10">
              <Bell className="w-6 h-6 text-primary" />
            </div>
            <h1 className="text-2xl font-bold text-text-primary">
              {i18n.title}
            </h1>
          </div>

          {/* Mark All as Read Button */}
          {ctrl.notifications && ctrl.notifications.length > 0 && (
            <button
              onClick={ctrl.handleMarkAllAsRead}
              disabled={ctrl.isMarkingAllAsRead}
              className="flex items-center gap-2 px-4 py-2 bg-primary text-background rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <CheckCircle className="w-4 h-4" />
              <span className="text-sm font-medium">
                {i18n.mark_all_as_read}
              </span>
            </button>
          )}
        </div>
      </div>

      {/* Empty State */}
      {!ctrl.notifications || ctrl.notifications.length === 0 ? (
        <div className="text-center py-16">
          <div className="inline-flex items-center justify-center w-20 h-20 rounded-full bg-primary/5 mb-4">
            <Bell className="w-10 h-10 text-foreground-muted" />
          </div>
          <h2 className="text-xl font-semibold mb-2 text-text-primary">
            {i18n.empty}
          </h2>
          <p className="text-foreground-muted text-sm max-w-md mx-auto">
            {i18n.empty_description}
          </p>
        </div>
      ) : (
        /* Notifications List */
        <div className="w-full space-y-0">
          {ctrl.notifications.map((notification) => {
            const notificationType = getNotificationType(
              notification.operation,
            );
            const notificationData = getNotificationData(
              notification.operation,
            ) as Record<string, string>;

            const timeAgo = formatDistanceToNow(
              new Date(notification.created_at),
              { addSuffix: true },
            );

            // Get the notification type label
            const typeKey = getNotificationTypeKey(notificationType);
            const typeLabel = i18n[typeKey] || i18n.unknown;

            // Determine notification content
            let notificationContent = '';
            let notificationTitle = '';

            if (
              notificationType === 'SpacePostNotification' &&
              notificationData
            ) {
              notificationContent = `${notificationData.author_display_name || 'Someone'} posted in your space`;
              notificationTitle = notificationData.post_title || '';
            } else if (notificationType === 'TeamInvite' && notificationData) {
              notificationContent = `You've been invited to join ${notificationData.team_name || 'a team'}`;
            } else if (notificationType === 'StartSurvey' && notificationData) {
              notificationContent = `${notificationData.author_display_name || 'Someone'} started a survey`;
              notificationTitle = notificationData.survey_title || '';
            } else if (
              notificationType === 'SpaceInviteVerification' &&
              notificationData
            ) {
              notificationContent = `${notificationData.author_display_name || 'Someone'} invited you to ${notificationData.space_title || 'a space'}`;
            } else {
              notificationContent = typeLabel;
            }

            return (
              <div
                key={notification.getNotificationId()}
                className="group flex items-start gap-4 py-4 border-b border-border-separator last:border-b-0 hover:bg-hover/50 transition-colors px-2 -mx-2 rounded-lg w-full"
              >
                {/* Avatar/Icon */}
                <div className="flex-shrink-0">
                  {notificationData?.author_profile ? (
                    <img
                      src={notificationData.author_profile}
                      alt=""
                      className="w-12 h-12 rounded-full object-cover"
                    />
                  ) : notificationData?.team_profile ? (
                    <img
                      src={notificationData.team_profile}
                      alt=""
                      className="w-12 h-12 rounded-full object-cover"
                    />
                  ) : (
                    <div className="w-12 h-12 rounded-full bg-primary flex items-center justify-center">
                      <Bell className="w-6 h-6 text-background" />
                    </div>
                  )}
                </div>

                {/* Content */}
                <div className="flex-1 min-w-0 pt-0.5">
                  <div className="flex items-start justify-between gap-2 mb-1">
                    <div className="flex items-center gap-2">
                      <p className="text-sm font-medium text-primary">
                        {typeLabel}
                      </p>
                      {notification.isUnread() && (
                        <span className="w-2 h-2 rounded-full bg-primary" />
                      )}
                    </div>
                    <span className="text-xs text-foreground-more-muted whitespace-nowrap">
                      {timeAgo}
                    </span>
                  </div>

                  {notificationContent && (
                    <p className="text-sm text-foreground mb-1 leading-relaxed">
                      {notificationContent}
                    </p>
                  )}

                  {notificationTitle && (
                    <p className="text-sm text-foreground-muted line-clamp-2">
                      {notificationTitle}
                    </p>
                  )}

                  {notificationData?.post_desc &&
                    notificationType === 'SpacePostNotification' && (
                      <p className="text-xs text-foreground-more-muted line-clamp-1 mt-1">
                        {notificationData.post_desc}
                      </p>
                    )}
                </div>

                {/* Action Buttons */}
                <div className="flex items-center gap-1 flex-shrink-0">
                  {/* Mark as Read Button - only show if unread */}
                  {notification.isUnread() && (
                    <button
                      onClick={() =>
                        ctrl.handleMarkAsRead(notification.getNotificationId())
                      }
                      disabled={ctrl.isMarkingAsRead}
                      className="p-2 hover:bg-card-bg rounded-lg transition-colors"
                      title={i18n.mark_as_read}
                    >
                      <CheckCircle className="w-4 h-4 text-foreground-muted hover:text-primary transition-colors" />
                    </button>
                  )}

                  {/* Delete Button */}
                  <button
                    onClick={() =>
                      ctrl.handleDelete(notification.getNotificationId())
                    }
                    disabled={ctrl.isDeleting}
                    className="p-2 hover:bg-card-bg rounded-lg transition-colors"
                    title={i18n.delete}
                  >
                    <Delete_2 className="w-4 h-4 text-foreground-muted hover:text-destructive transition-colors" />
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
