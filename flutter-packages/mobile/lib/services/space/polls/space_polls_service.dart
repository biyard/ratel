import 'package:ratel/exports.dart';

class SpacePollsService extends GetxService {
  static void init() {
    Get.put<SpacePollsService>(SpacePollsService());
    Get.put<SpacePollsApi>(SpacePollsApi());
  }
}
