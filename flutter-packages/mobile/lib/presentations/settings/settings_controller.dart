import 'package:ratel/exports.dart';

class SettingsController extends BaseController {
  void goBack() => Get.back();

  Future<void> logout() async {
    try {
      await Get.find<AuthApi>().logout();

      final auth = Get.find<AuthService>();
      auth.accounts.clear();
      auth.accountsBookmark.value = null;
      auth.accountsError.value = null;
      auth.accountsBusy.value = false;
      auth.currentUserPk.value = null;

      await auth.loadAccounts(refresh: true);

      if (Get.isRegistered<AccountController>()) {
        Get.delete<AccountController>(force: true);
      }

      Biyard.info('Success to logout. You will go to the login page.');
      Get.rootDelegate.offNamed(accountScreen);
    } catch (e) {
      logger.e('logout failed: $e');
      Biyard.error(
        'Logout Failed',
        'Failed to logout. Please try again later.',
      );
    }
  }
}
