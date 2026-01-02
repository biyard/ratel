import 'package:ratel/exports.dart';

enum LoginMethod { phone, email }

class LoginController extends BaseController {
  final signupService = Get.find<SignupService>();

  final isBusy = false.obs;
  final showPassword = false.obs;

  final method = LoginMethod.phone.obs;

  final emailCtrl = TextEditingController();
  final passwordCtrl = TextEditingController();
  final phoneCtrl = TextEditingController();

  final email = ''.obs;
  final password = ''.obs;
  final phone = ''.obs;

  final selectedCountry = CountryCode(
    code: 'KR',
    name: 'Republic of Korea',
    dialCode: '82',
  ).obs;

  final isPhoneValid = false.obs;
  final showWarning = false.obs;

  bool get isPhone => method.value == LoginMethod.phone;
  bool get isEmail => method.value == LoginMethod.email;

  bool get isFormValid {
    if (isPhone) return isPhoneValid.value;
    return email.value.trim().isNotEmpty && password.value.trim().isNotEmpty;
  }

  void selectMethod(LoginMethod m) {
    method.value = m;
    showWarning.value = false;
  }

  void selectCountry(CountryCode code) => selectedCountry.value = code;

  void onPhoneChanged(String v) {
    final digits = v.replaceAll(RegExp(r'[^0-9]'), '');
    phone.value = digits;
    isPhoneValid.value = digits.length >= 6;

    phoneCtrl.value = phoneCtrl.value.copyWith(
      text: digits,
      selection: TextSelection.collapsed(offset: digits.length),
    );

    if (showWarning.value) showWarning.value = !isFormValid;
  }

  void onEmailChanged(String v) {
    email.value = v;
    if (showWarning.value) showWarning.value = !isFormValid;
  }

  void onPasswordChanged(String v) {
    password.value = v;
    if (showWarning.value) showWarning.value = !isFormValid;
  }

  void markWarningIfInvalid() {
    showWarning.value = !isFormValid;
  }

  Future<void> submit() async {
    if (isBusy.value) return;

    if (!isFormValid) {
      markWarningIfInvalid();
      return;
    }

    isBusy.value = true;
    try {
      showWarning.value = false;

      if (isEmail) {
        final authService = Get.find<AuthService>();
        final auth = Get.find<AuthApi>();
        await auth.clearSession();

        final e = email.value.trim();
        final p = password.value.trim();
        final res = await auth.loginWithPassword(e, p);
        await authService.loadAccounts(refresh: false);

        if (res != null) {
          Get.rootDelegate.offNamed(AppRoutes.mainScreen);
          Biyard.info("Login Successed.");
        } else {
          Biyard.error(
            "Failed to login",
            "Login failed. Please try again later.",
          );
        }
        return;
      }

      final auth = AuthApi();
      final fullPhone = '+${selectedCountry.value.dialCode}${phone.value}';
      final res = await auth.sendVerificationCode(fullPhone);

      if (res != null) {
        signupService.phone(fullPhone);
        Get.rootDelegate.toNamed(verificationScreen);
      } else {
        Biyard.error(
          "Failed to send authorization code",
          "Send Authorization code failed. Please try again later.",
        );
      }

      Biyard.info('Phone: $fullPhone');
    } finally {
      isBusy.value = false;
    }
  }

  void toggleShowPassword() => showPassword.toggle();

  void goToSignup() {
    Get.rootDelegate.offNamed(signupScreen);
  }

  @override
  void onClose() {
    emailCtrl.dispose();
    passwordCtrl.dispose();
    phoneCtrl.dispose();
    super.onClose();
  }
}
