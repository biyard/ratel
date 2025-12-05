import 'package:ratel/exports.dart';
import 'package:ratel/presentations/notification/components/notification_header.dart';
import 'package:ratel/presentations/notification/components/notification_list.dart';

class NotificationScreen extends GetWidget<NotificationController> {
  const NotificationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<NotificationController>(
      enableSafeArea: false,
      scrollable: false,
      child: SafeArea(
        bottom: false,
        child: Column(
          children: [
            const Header(title: 'Notification'),
            15.vgap,
            NotificationHeader(onMarkAllRead: () => controller.markAllAsRead()),
            Expanded(
              child: Obx(() {
                final items = controller.items.toList();
                final isLoading = controller.isLoading.value;
                final isLoadingMore = controller.isLoadingMore.value;
                final hasMore = controller.hasMore;

                return RefreshIndicator(
                  onRefresh: controller.refreshNotifications,
                  color: AppColors.primary,
                  backgroundColor: AppColors.bg,
                  child: NotificationList(
                    items: items,
                    isLoading: isLoading,
                    isLoadingMore: isLoadingMore,
                    hasMore: hasMore,
                    bottomPadding: bottomPad,
                    onLoadMore: controller.loadMore,
                    onMarkRead: controller.markNotificationAsRead,
                    onDelete: controller.deleteNotification,
                  ),
                );
              }),
            ),
          ],
        ),
      ),
    );
  }
}
