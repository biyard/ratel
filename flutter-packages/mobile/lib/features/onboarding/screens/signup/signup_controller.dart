import 'package:ratel/exports.dart';

class SignupController extends BaseController {
  final signupService = Get.find<SignupService>();

  final phoneCtrl = TextEditingController();

  final isBusy = false.obs;

  final phone = ''.obs;

  final selectedCountry = Rx<CountryCode>(kDefaultCountryCode);

  final isPhoneValid = false.obs;

  String get fullPhoneNumber =>
      '+${selectedCountry.value.dialCode}${phone.value}';

  void onPhoneChanged(String v) {
    final digits = v.replaceAll(RegExp(r'[^0-9]'), '');
    phone.value = digits;
    isPhoneValid.value = digits.length >= 6;
    phoneCtrl.value = phoneCtrl.value.copyWith(
      text: digits,
      selection: TextSelection.collapsed(offset: digits.length),
    );
  }

  void selectCountry(CountryCode c) => selectedCountry.value = c;

  bool get isPhoneStepValid => isPhoneValid.value;

  @override
  void onInit() {
    super.onInit();
    phoneCtrl.text = phone.value;
  }

  void goBack() {
    Get.rootDelegate.offNamed(loginScreen);
  }

  Future<void> next() async {
    if (!isPhoneStepValid || isBusy.value) return;

    final auth = AuthApi();
    isBusy.value = true;
    try {
      final res = await auth.sendVerificationCode(fullPhoneNumber);

      if (res != null) {
        signupService.phone(fullPhoneNumber);
        Get.rootDelegate.offNamed(verificationScreen);
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

  void restartSignup() {
    phone.value = '';
    phoneCtrl.clear();
    selectedCountry.value = kDefaultCountryCode;
  }

  @override
  void onClose() {
    phoneCtrl.dispose();
    super.onClose();
  }
}
