import 'package:ratel/exports.dart';

class SpaceFilesService extends GetxService {
  static void init() {
    Get.put<SpaceFilesService>(SpaceFilesService());
    Get.put<SpaceFilesApi>(SpaceFilesApi());
  }
}
