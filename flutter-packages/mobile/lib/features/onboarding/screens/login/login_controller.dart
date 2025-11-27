// login_controller.dart
import 'package:ratel/exports.dart';

class LoginController extends BaseController {
  final signupService = Get.find<SignupService>();
  final emailCtrl = TextEditingController();
  final passwordCtrl = TextEditingController();
  final isBusy = false.obs;
  final showPassword = false.obs;
  final email = ''.obs;
  final password = ''.obs;

  bool get isFormValid => email.isNotEmpty && password.isNotEmpty;

  void toggleShowPassword() => showPassword.toggle();

  Future<void> signIn() async {
    final auth = Get.find<AuthApi>();
    if (isBusy.value || !isFormValid) return;
    isBusy.value = true;
    try {
      await auth.clearSession();
      final res = await auth.loginWithPassword(email.value, password.value);
      if (res != null) {
        Get.rootDelegate.offNamed(AppRoutes.mainScreen);
      } else {
        Biyard.error(
          "Failed to login",
          "Login failed. Please try again later.",
        );
      }
    } finally {
      isBusy.value = false;
    }
  }

  void goToSignup() {
    Get.rootDelegate.offNamed(signupScreen);
  }

  @override
  void onClose() {
    emailCtrl.dispose();
    passwordCtrl.dispose();
    super.onClose();
  }
}
