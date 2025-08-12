import 'package:ratel/exports.dart';

class NetworkService extends GetxService {
  static void init() {
    Get.put<NetworkService>(NetworkService());
    Get.put<NetworkApi>(NetworkApi());
  }
}
