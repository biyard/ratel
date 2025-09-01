import 'package:ratel/exports.dart';

class DashboardsService extends GetxService {
  static void init() {
    Get.put<DashboardsService>(DashboardsService());
    Get.put<DashboardsApi>(DashboardsApi());
  }
}
