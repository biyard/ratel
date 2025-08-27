import 'package:ratel/exports.dart';

class NotificationController extends BaseController {
  final networkApi = Get.find<NetworkApi>();
  final notificationsApi = Get.find<NotificationApi>();

  RxList<NotificationFollower> items = <NotificationFollower>[].obs;

  @override
  void onInit() {
    super.onInit();
    getNotifications();
  }

  Future<void> getNotifications() async {
    showLoading();
    try {
      final data = await notificationsApi.getNotifications();
      items.assignAll(data.networks);
    } finally {
      hideLoading();
    }
  }

  Future<void> acceptInvitation(int followeeId) async {
    final res = await networkApi.acceptInvitation([], followeeId);
    if (res != null) {
      await getNotifications();
      Biyard.info('Invitation accepted.');
    } else {
      Biyard.error(
        'Failed to accept invitation.',
        'Accept invitation is failed. Please try again later.',
      );
    }
  }

  Future<void> rejectInvitation(int followeeId) async {
    final res = await networkApi.rejectInvitation([], followeeId);
    if (res != null) {
      await getNotifications();
      Biyard.info('Invitation rejected.');
    } else {
      Biyard.error(
        'Failed to reject invitation.',
        'Reject invitation is failed. Please try again later.',
      );
    }
  }
}
