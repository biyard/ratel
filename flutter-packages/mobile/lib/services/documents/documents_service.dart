import 'package:ratel/exports.dart';

class DocumentsService extends GetxService {
  static void init() {
    Get.put<DocumentsService>(DocumentsService());
    Get.put<DashboardsApi>(DashboardsApi());
  }
}
