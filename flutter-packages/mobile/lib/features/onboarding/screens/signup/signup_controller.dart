import 'package:ratel/exports.dart';

enum SignupMethod { phone, email }

class SignupController extends BaseController {
  final signupService = Get.find<SignupService>();

  final isBusy = false.obs;

  final method = SignupMethod.phone.obs;

  final phoneCtrl = TextEditingController();
  final emailCtrl = TextEditingController();
  final passwordCtrl = TextEditingController();

  final phone = ''.obs;
  final email = ''.obs;
  final password = ''.obs;

  final selectedCountry = Rx<CountryCode>(kDefaultCountryCode);

  final isPhoneValid = false.obs;

  final showWarning = false.obs;

  bool get isPhone => method.value == SignupMethod.phone;
  bool get isEmail => method.value == SignupMethod.email;

  String get fullPhoneNumber =>
      '+${selectedCountry.value.dialCode}${phone.value}';

  bool get isPhoneStepValid => isPhoneValid.value;

  bool get isEmailStepValid =>
      email.value.trim().isNotEmpty && password.value.trim().isNotEmpty;

  bool get isCurrentStepValid => isPhone ? isPhoneStepValid : isEmailStepValid;

  void selectMethod(SignupMethod m) {
    method.value = m;
    showWarning.value = false;
  }

  void onPhoneChanged(String v) {
    final digits = v.replaceAll(RegExp(r'[^0-9]'), '');
    phone.value = digits;
    isPhoneValid.value = digits.length >= 6;

    phoneCtrl.value = phoneCtrl.value.copyWith(
      text: digits,
      selection: TextSelection.collapsed(offset: digits.length),
    );

    if (showWarning.value) {
      showWarning.value = !isCurrentStepValid;
    }
  }

  void onEmailChanged(String v) {
    email.value = v.trim();
    if (showWarning.value) {
      showWarning.value = !isCurrentStepValid;
    }
  }

  void onPasswordChanged(String v) {
    password.value = v.trim();
    if (showWarning.value) {
      showWarning.value = !isCurrentStepValid;
    }
  }

  void selectCountry(CountryCode c) => selectedCountry.value = c;

  @override
  void onInit() {
    super.onInit();
    phoneCtrl.text = phone.value;
    emailCtrl.text = email.value;
    passwordCtrl.text = password.value;
  }

  void goToLogin() {
    Get.rootDelegate.offNamed(accountScreen);
  }

  void onContinueTap() {
    if (isBusy.value) return;

    if (!isCurrentStepValid) {
      showWarning.value = true;
      return;
    }

    showWarning.value = false;

    if (isPhone) {
      nextPhone();
    } else {
      nextEmail();
    }
  }

  Future<void> nextPhone() async {
    if (isBusy.value) return;

    if (!isPhoneStepValid) {
      showWarning.value = true;
      return;
    }

    final auth = AuthApi();
    isBusy.value = true;
    try {
      final res = await auth.sendVerificationCode(fullPhoneNumber);

      if (res != null) {
        signupService.phone(fullPhoneNumber);
        Get.rootDelegate.toNamed(verificationScreen);
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

  Future<void> nextEmail() async {
    if (isBusy.value) return;

    if (!isEmailStepValid) {
      showWarning.value = true;
      return;
    }

    isBusy.value = true;
    try {
      Biyard.info('Signup email=${email.value}');
    } finally {
      isBusy.value = false;
    }
  }

  @override
  void onClose() {
    phoneCtrl.dispose();
    emailCtrl.dispose();
    passwordCtrl.dispose();
    super.onClose();
  }
}
