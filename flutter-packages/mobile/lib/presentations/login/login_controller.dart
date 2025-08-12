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

  Future<void> signIn() async {
    if (isBusy.value || !isFormValid) return;
    isBusy.value = true;
    try {
      await Future.delayed(const Duration(milliseconds: 800));
      Get.rootDelegate.offNamed(AppRoutes.mainScreen);
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
