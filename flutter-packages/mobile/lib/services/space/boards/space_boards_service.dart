import 'package:ratel/exports.dart';

class SpaceBoardsService extends GetxService {
  static void init() {
    Get.put<SpaceBoardsService>(SpaceBoardsService());
    Get.put<SpaceBoardsApi>(SpaceBoardsApi());
  }
}
