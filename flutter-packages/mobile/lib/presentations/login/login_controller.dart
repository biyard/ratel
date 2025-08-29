// login_controller.dart
import 'package:ratel/exports.dart';

class LoginController extends BaseController {
  final emailCtrl = TextEditingController();
  final passwordCtrl = TextEditingController();

  final isBusy = false.obs;
  final showPassword = false.obs;

  final email = ''.obs;
  final password = ''.obs;

  bool get isFormValid => email.isNotEmpty && password.isNotEmpty;

  void toggleShowPassword() => showPassword.toggle();

  @override
  void onInit() {
    super.onInit();
    autoLogin();
  }

  Future<void> autoLogin() async {
    final auth = AuthApi();
    await auth.init();
    final ok = await auth.tryAutoSignIn();
    if (ok) {
      Get.rootDelegate.offNamed(AppRoutes.mainScreen);
    }
  }

  Future<void> signIn() async {
    final auth = AuthApi();
    if (isBusy.value || !isFormValid) return;
    isBusy.value = true;
    try {
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

  Future<void> signInWithGoogle() async {
    Get.snackbar('Google', 'Sign-in pressed');
  }

  void goToSignup() {
    Get.rootDelegate.offNamed(AppRoutes.signupScreen);
  }

  @override
  void onClose() {
    emailCtrl.dispose();
    passwordCtrl.dispose();
    super.onClose();
  }
}
