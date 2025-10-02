import 'package:ratel/exports.dart';

class SettingsController extends BaseController {
  void goBack() => Get.back();
  Future<void> logout() async {
    try {
      await Get.find<AuthApi>().logout();
      Get.rootDelegate.offNamed(AppRoutes.loginScreen);
    } catch (e) {
      logger.e('logout failed: $e');
    }
  }
}
