import 'package:ratel/exports.dart';

class SpaceService extends GetxService {
  static void init() {
    Get.put<SpaceService>(SpaceService());
    Get.put<SpaceApi>(SpaceApi());
  }
}
