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

  bool get isFormFilled =>
      email.isNotEmpty &&
      password.isNotEmpty &&
      confirm.isNotEmpty &&
      password.value == confirm.value &&
      GetUtils.isEmail(email.value);

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
    if (!isFormFilled || isBusy.value) return;
    isBusy.value = true;
    try {
      await Future.delayed(const Duration(milliseconds: 800));
      Get.rootDelegate.offNamed(AppRoutes.verificationScreen);
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
