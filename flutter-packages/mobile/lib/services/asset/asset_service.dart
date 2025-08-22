import 'package:ratel/exports.dart';

class AssetService extends GetxService {
  static void init() {
    Get.put<AssetService>(AssetService());
    Get.put<AssetApi>(AssetApi());
  }
}
