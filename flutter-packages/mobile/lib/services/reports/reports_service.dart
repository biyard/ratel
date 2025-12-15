import 'package:ratel/exports.dart';

class ReportsService extends GetxService {
  static void init() {
    Get.put<ReportApi>(ReportApi());
    Get.put<ReportsService>(ReportsService());
  }
}
