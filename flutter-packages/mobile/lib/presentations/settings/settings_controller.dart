import 'package:ratel/exports.dart';

class SettingsController extends BaseController {
  void goBack() => Get.back();
  Future<void> logout() async {
    try {
      await Get.find<AuthApi>().logout();
      Biyard.info('Success to logout. You will go to the login page.');
      Get.rootDelegate.offNamed(loginScreen);
    } catch (e) {
      logger.e('logout failed: $e');
      Biyard.error(
        'Logout Failed',
        'Failed to logout. Please try again later.',
      );
    }
  }
}
