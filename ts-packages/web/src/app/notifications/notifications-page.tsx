import { useNotificationsI18n } from '@/features/notification/i18n';
import { useNotificationsPageController } from '@/app/notifications/use-notifications-page-controller';
import { Bell, Delete_2, Check, CheckCircle } from '@/components/icons';
import {
  getNotificationType,
  getNotificationData,
} from '@/features/notification/types/email-operation';
import { Button } from '@/components/ui/button';
import Card from '@/components/card';

export default function NotificationsPage() {
  const ctrl = useNotificationsPageController();
  const i18n = useNotificationsI18n();

  if (ctrl.isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-text-primary">{i18n.title}...</div>
      </div>
    );
  }

  if (ctrl.error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-red-500">
          Error: {ctrl.error.message}
        </div>
      </div>
    );
  }

  const unreadCount =
    ctrl.notifications?.filter((n) => n.isUnread()).length || 0;

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center gap-3">
          <Bell className="w-8 h-8" />
          <div>
            <h1 className="text-3xl font-bold text-text-primary">
              {i18n.title}
            </h1>
            {unreadCount > 0 && (
              <p className="text-sm text-gray-500 dark:text-gray-400">
                {unreadCount} {i18n.unread}
              </p>
            )}
          </div>
        </div>
        {ctrl.notifications &&
          ctrl.notifications.length > 0 &&
          unreadCount > 0 && (
            <Button
              onClick={ctrl.handleMarkAllAsRead}
              variant="outline"
              size="sm"
              disabled={ctrl.isMarkingAllAsRead}
            >
              <CheckCircle className="w-4 h-4 mr-2" />
              {i18n.mark_all_as_read}
            </Button>
          )}
      </div>

      {!ctrl.notifications || ctrl.notifications.length === 0 ? (
        <Card className="p-12 text-center">
          <Bell className="w-16 h-16 mx-auto mb-4 text-gray-400 dark:text-gray-600" />
          <h2 className="text-xl font-semibold mb-2 text-text-primary">
            {i18n.empty}
          </h2>
          <p className="text-gray-600 dark:text-gray-400">
            {i18n.empty_description}
          </p>
        </Card>
      ) : (
        <div className="space-y-2">
          {ctrl.notifications.map((notification) => {
            const notificationType = getNotificationType(
              notification.operation,
            );
            const notificationData = getNotificationData(
              notification.operation,
            ) as Record<string, string>;

            return (
              <Card
                key={notification.getNotificationId()}
                className={`p-4 transition-colors hover:bg-gray-50 dark:hover:bg-gray-800 ${
                  notification.isUnread()
                    ? 'bg-blue-50 dark:bg-blue-900/20'
                    : ''
                }`}
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-2">
                      {notification.isUnread() && (
                        <span className="w-2 h-2 bg-blue-500 rounded-full flex-shrink-0"></span>
                      )}
                      <h3 className="font-semibold text-sm truncate text-text-primary">
                        {i18n[
                          notificationType.toLowerCase() as keyof typeof i18n
                        ] || i18n.unknown}
                      </h3>
                      <span className="text-xs text-gray-500 dark:text-gray-400 flex-shrink-0">
                        {new Date(notification.created_at).toLocaleDateString()}
                      </span>
                    </div>

                    {notificationType === 'SpacePostNotification' &&
                      notificationData && (
                        <div className="text-sm text-gray-800 dark:text-gray-300">
                          <p className="mb-1">
                            <span className="font-medium text-text-primary">
                              {notificationData.author_display_name}
                            </span>{' '}
                            posted: {notificationData.post_title}
                          </p>
                          <p className="text-gray-600 dark:text-gray-500 line-clamp-2">
                            {notificationData.post_desc}
                          </p>
                        </div>
                      )}

                    {notificationType === 'TeamInvite' && notificationData && (
                      <div className="text-sm text-gray-800 dark:text-gray-300">
                        <p>
                          You've been invited to join{' '}
                          <span className="font-medium text-text-primary">
                            {notificationData.team_name}
                          </span>
                        </p>
                      </div>
                    )}

                    {notificationType === 'StartSurvey' && notificationData && (
                      <div className="text-sm text-gray-800 dark:text-gray-300">
                        <p className="mb-1">
                          <span className="font-medium text-text-primary">
                            {notificationData.author_display_name}
                          </span>{' '}
                          started a survey: {notificationData.survey_title}
                        </p>
                        <p className="text-gray-600 dark:text-gray-500">
                          in {notificationData.space_title}
                        </p>
                      </div>
                    )}
                  </div>

                  <div className="flex gap-2 flex-shrink-0">
                    {notification.isUnread() && (
                      <Button
                        onClick={() =>
                          ctrl.handleMarkAsRead(
                            notification.getNotificationId(),
                          )
                        }
                        variant="text"
                        size="sm"
                        disabled={ctrl.isMarkingAsRead}
                        title={i18n.mark_as_read}
                        className="h-8 w-8 p-0 min-w-0"
                      >
                        <Check className="w-4 h-4" />
                      </Button>
                    )}
                    <Button
                      onClick={() =>
                        ctrl.handleDelete(notification.getNotificationId())
                      }
                      variant="text"
                      size="sm"
                      disabled={ctrl.isDeleting}
                      title={i18n.delete}
                      className="h-8 w-8 p-0 min-w-0"
                    >
                      <Delete_2 className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}
