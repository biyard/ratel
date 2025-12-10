import 'package:ratel/exports.dart';

class NotificationController extends BaseController {
  final notificationsApi = Get.find<NotificationApi>();

  RxList<AppNotification> items = <AppNotification>[].obs;
  final bookmark = RxnString();
  final isLoading = false.obs;
  final isLoadingMore = false.obs;

  bool get hasMore => bookmark.value != null && bookmark.value!.isNotEmpty;

  @override
  void onInit() {
    super.onInit();
    loadInitial();
  }

  Future<void> loadInitial() async {
    isLoading.value = true;
    try {
      bookmark.value = null;
      final page = await notificationsApi.getNotifications();
      items.assignAll(page.items);
      bookmark.value = page.bookmark;
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> refreshNotifications() async {
    await loadInitial();
  }

  Future<void> loadMore() async {
    if (isLoadingMore.value || !hasMore) return;

    isLoadingMore.value = true;
    try {
      final page = await notificationsApi.getNotifications(
        bookmark: bookmark.value,
      );
      items.addAll(page.items);
      bookmark.value = page.bookmark;
    } finally {
      isLoadingMore.value = false;
    }
  }

  Future<void> markNotificationAsRead(AppNotification notification) async {
    final res = await notificationsApi.markAsRead([notification.sk]);
    if (res.success) {
      await loadInitial();
      Biyard.info('Marked as read.');
    } else {
      Biyard.error('Failed to mark as read.', 'Please try again later.');
    }
  }

  Future<void> markAllAsRead() async {
    final res = await notificationsApi.markAllAsRead();
    if (res.success) {
      await loadInitial();
      Biyard.info('All notifications marked as read.');
    } else {
      Biyard.error('Failed to mark all as read.', 'Please try again later.');
    }
  }

  Future<void> deleteNotification(AppNotification notification) async {
    final res = await notificationsApi.deleteNotification(
      notification.notificationId,
    );
    if (res.success) {
      await loadInitial();
      Biyard.info('Notification deleted.');
    } else {
      Biyard.error('Failed to delete notification.', 'Please try again later.');
    }
  }
}
