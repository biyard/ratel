import 'package:ratel/exports.dart';

class NotificationsService extends GetxService {
  static void init() {
    Get.put<NotificationsService>(NotificationsService());
    Get.put<NotificationApi>(NotificationApi());
  }
}
