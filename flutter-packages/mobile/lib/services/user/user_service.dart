import 'package:ratel/exports.dart';

class UserService extends GetxService {
  static void init() {
    Get.put<UserService>(UserService());
    Get.put<UserApi>(UserApi());
  }
}
