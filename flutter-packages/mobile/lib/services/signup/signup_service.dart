import 'package:ratel/exports.dart';

class SignupService extends GetxService {
  static Future<void> init() async {
    Get.put<SignupService>(SignupService());
  }

  Rx<String> phone = "".obs;
  Rx<String> email = "".obs;
  Rx<String> password = "".obs;
  Rx<String> displayName = "".obs;
  Rx<String> username = "".obs;
  Rx<String> confirm = "".obs;
}
