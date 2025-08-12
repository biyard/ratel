import 'package:ratel/exports.dart';

class SignupController extends BaseController {
  final signupService = Get.find<SignupService>();

  final emailCtrl = TextEditingController();
  final passwordCtrl = TextEditingController();
  final confirmCtrl = TextEditingController();

  final isBusy = false.obs;

  final showPassword = false.obs;
  final showConfirm = false.obs;

  Rx<String> get email => signupService.email;
  Rx<String> get password => signupService.password;
  Rx<String> get confirm => signupService.confirm;

  bool isStrongPassword(String s) {
    final pwd = s.trim();
    if (pwd.length < 8) return false;

    final hasLetter = RegExp(r'[A-Za-z]').hasMatch(pwd);
    final hasDigit = RegExp(r'\d').hasMatch(pwd);
    final hasSpecial = RegExp(r'[^A-Za-z0-9]').hasMatch(pwd);

    return hasLetter && hasDigit && hasSpecial;
  }

  bool get isFormFilled =>
      email.value.isNotEmpty &&
      password.value.isNotEmpty &&
      confirm.value.isNotEmpty &&
      password.value == confirm.value &&
      GetUtils.isEmail(email.value) &&
      isStrongPassword(password.value);

  void onEmailChanged(String v) => email.value = v.trim();
  void onPasswordChanged(String v) => password.value = v;
  void onConfirmChanged(String v) => confirm.value = v;

  void toggleShowPassword() => showPassword.toggle();
  void toggleShowConfirm() => showConfirm.toggle();

  @override
  void onInit() {
    super.onInit();
    emailCtrl.text = email.value;
    passwordCtrl.text = password.value;
    confirmCtrl.text = confirm.value;
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.loginScreen);
  }

  Future<void> next() async {
    final auth = AuthApi();
    if (!isFormFilled || isBusy.value) return;
    isBusy.value = true;
    try {
      final res = await auth.sendVerificationCode(email.value);

      if (res != null) {
        Get.rootDelegate.offNamed(AppRoutes.verificationScreen);
      } else {
        Biyard.error(
          "Failed to send authorization code",
          "Send Authorization code failed. Please try again later.",
        );
      }
    } finally {
      isBusy.value = false;
    }
  }

  @override
  void onClose() {
    emailCtrl.dispose();
    passwordCtrl.dispose();
    confirmCtrl.dispose();
    super.onClose();
  }
}
