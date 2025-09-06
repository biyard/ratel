import 'package:ratel/exports.dart';

class FeedsService extends GetxService {
  static void init() {
    Get.put<FeedsService>(FeedsService());
    Get.put<FeedsApi>(FeedsApi());
  }
}
