// login_controller.dart
import 'dart:io';

import 'package:google_sign_in/google_sign_in.dart';
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

  Future<void> signInWithApple() async {
    logger.d("apple login clicked");
  }

  Future<void> signInWithGoogle() async {
    final auth = Get.find<AuthService>();
    if (isBusy.value) return;
    isBusy.value = true;

    final signIn = await auth.connectToGoogle("");
    logger.d("user: ${signIn}");
    // try {

    // } catch (e, st) {
    //   logger.e('Google Sign-In error: ${e}');
    //   Biyard.error('Google Sign-In', '$e');
    // } finally {
    //   isBusy.value = false;
    // }
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
