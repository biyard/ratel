import 'package:ratel/exports.dart';

class IndustryService extends GetxService {
  static void init() {
    Get.put<IndustryService>(IndustryService());
    Get.put<IndustryApi>(IndustryApi());
  }
}
