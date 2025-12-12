import 'package:ratel/exports.dart';

class NotificationList extends StatelessWidget {
  const NotificationList({
    super.key,
    required this.items,
    required this.isLoading,
    required this.isLoadingMore,
    required this.hasMore,
    required this.bottomPadding,
    required this.onLoadMore,
    required this.onMarkRead,
    required this.onDelete,
  });

  final List<AppNotification> items;
  final bool isLoading;
  final bool isLoadingMore;
  final bool hasMore;
  final double bottomPadding;
  final VoidCallback onLoadMore;
  final void Function(AppNotification) onMarkRead;
  final void Function(AppNotification) onDelete;

  @override
  Widget build(BuildContext context) {
    if (isLoading && items.isEmpty) {
      return const Padding(
        padding: EdgeInsets.only(top: 40),
        child: Center(
          child: SizedBox(
            width: 24,
            height: 24,
            child: CircularProgressIndicator(strokeWidth: 2),
          ),
        ),
      );
    }

    if (items.isEmpty) {
      return Padding(
        padding: EdgeInsets.only(top: 80, bottom: bottomPadding + 16),
        child: Center(
          child: Text(
            'No notifications yet.',
            style: Theme.of(
              context,
            ).textTheme.bodyMedium?.copyWith(color: AppColors.neutral500),
          ),
        ),
      );
    }

    return Column(
      children: [
        ListView.separated(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          padding: EdgeInsets.fromLTRB(0, 8, 0, bottomPadding + 16),
          itemCount: items.length,
          separatorBuilder: (_, __) {
            return const Padding(
              padding: EdgeInsets.symmetric(horizontal: 16),
              child: SizedBox(
                width: double.infinity,
                height: 1,
                child: ColoredBox(color: Color(0xff464646)),
              ),
            );
          },
          itemBuilder: (context, index) {
            final item = items[index];

            return NotificationListItem(
              notification: item,
              onTap: () => onMarkRead(item),
              onMarkRead: item.isUnread ? () => onMarkRead(item) : null,
              onDelete: () => onDelete(item),
            );
          },
        ),
        if (hasMore)
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: isLoadingMore
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const SizedBox(height: 15),
          ),
      ],
    );
  }
}

class NotificationListItem extends StatelessWidget {
  const NotificationListItem({
    super.key,
    required this.notification,
    this.onTap,
    this.onMarkRead,
    this.onDelete,
  });

  final AppNotification notification;
  final VoidCallback? onTap;
  final VoidCallback? onMarkRead;
  final VoidCallback? onDelete;

  @override
  Widget build(BuildContext context) {
    final op = notification.operation;
    final title = _NotificationTextHelper.titleFor(op);
    final message = _NotificationTextHelper.messageFor(op);
    final dt = DateTime.fromMillisecondsSinceEpoch(notification.createdAt);
    final createdText = formatRelativeTime(dt);
    final isUnread = notification.isUnread;

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Text(
                      title,
                      style: const TextStyle(
                        color: AppColors.primary,
                        fontSize: 13,
                        fontWeight: FontWeight.w700,
                        height: 1.2,
                      ),
                    ),
                    // if (isUnread) ...[
                    //   5.gap,
                    //   Container(
                    //     width: 6,
                    //     height: 6,
                    //     decoration: const BoxDecoration(
                    //       color: AppColors.primary,
                    //       shape: BoxShape.circle,
                    //     ),
                    //   ),
                    // ],
                  ],
                ),
                5.vgap,
                Text(
                  message,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w500,
                    height: 1.3,
                  ),
                ),
                10.vgap,
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      createdText,
                      style: const TextStyle(
                        color: AppColors.neutral500,
                        fontSize: 12,
                        fontWeight: FontWeight.w500,
                        height: 1.2,
                      ),
                    ),
                    // Row(
                    //   children: [
                    //     if (onMarkRead != null)
                    //       InkWell(
                    //         onTap: onMarkRead,
                    //         child: Icon(
                    //           notification.isRead
                    //               ? Icons.check_circle
                    //               : Icons.check_circle_outline,
                    //           size: 18,
                    //           color: notification.isRead
                    //               ? AppColors.primary
                    //               : AppColors.neutral500,
                    //         ),
                    //       ),
                    //     20.gap,
                    //     if (onDelete != null)
                    //       InkWell(
                    //         onTap: onDelete,
                    //         child: const Icon(
                    //           Icons.delete_outline,
                    //           size: 18,
                    //           color: AppColors.neutral500,
                    //         ),
                    //       ),
                    //   ],
                    // ),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _NotificationTextHelper {
  static String titleFor(EmailOperation op) {
    if (op is SpacePostNotificationOperation) {
      return 'Space update';
    }
    if (op is TeamInviteOperation) {
      return 'Team invite';
    }
    if (op is SpaceInviteVerificationOperation) {
      return 'Space invite';
    }
    if (op is SignupSecurityCodeOperation) {
      return 'Security code';
    }
    if (op is StartSurveyOperation) {
      return 'Survey';
    }
    return 'Notification';
  }

  static String messageFor(EmailOperation op) {
    if (op is SpacePostNotificationOperation) {
      if (op.postTitle.isNotEmpty) {
        return '${op.authorDisplayName} posted: ${op.postTitle}';
      }
      return '${op.authorDisplayName} shared a new post.';
    }
    if (op is TeamInviteOperation) {
      if (op.teamDisplayName.isNotEmpty) {
        return '${op.teamDisplayName} invited you to join ${op.teamName}.';
      }
      return 'You received a team invitation.';
    }
    if (op is SpaceInviteVerificationOperation) {
      if (op.spaceTitle.isNotEmpty) {
        return '${op.authorDisplayName} invited you to ${op.spaceTitle}.';
      }
      return 'You received a space invitation.';
    }
    if (op is SignupSecurityCodeOperation) {
      return 'Your signup verification code is ready.';
    }
    if (op is StartSurveyOperation) {
      if (op.surveyTitle.isNotEmpty) {
        return '${op.spaceTitle}: ${op.surveyTitle} has started.';
      }
      return 'A new survey has started.';
    }
    return 'You have a new notification.';
  }
}
