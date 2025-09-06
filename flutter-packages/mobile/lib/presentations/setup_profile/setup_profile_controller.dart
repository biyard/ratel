import 'package:ratel/exports.dart';

class SetupProfileController extends BaseController {
  final signupService = Get.find<SignupService>();

  final displayNameController = TextEditingController();
  final usernameController = TextEditingController();

  final termsAccepted = false.obs;
  final formValid = false.obs;

  Rx<String> get displayName => signupService.displayName;
  Rx<String> get username => signupService.username;
  Rx<String> get email => signupService.email;
  Rx<String> get password => signupService.password;
  Rx<String> avatarUrl = "".obs;

  @override
  void onInit() {
    super.onInit();

    displayNameController.text = displayName.value;
    usernameController.text = username.value;

    displayNameController.addListener(_syncDisplayName);
    usernameController.addListener(_syncUsername);

    _recomputeValid();
  }

  void _syncDisplayName() {
    displayName.value = displayNameController.text.trim();
    _recomputeValid();
  }

  void _syncUsername() {
    username.value = usernameController.text.trim();
    _recomputeValid();
  }

  void toggleTerms() {
    termsAccepted.toggle();
    _recomputeValid();
  }

  void _recomputeValid() {
    formValid.value =
        displayName.value.isNotEmpty &&
        username.value.isNotEmpty &&
        termsAccepted.value;
  }

  Future<void> next() async {
    final auth = AuthApi();
    if (!formValid.value) return;
    logger.d('Setup Profile: ${displayName.value}, ${username.value}');

    try {
      final res = await auth.signup(
        email.value,
        password.value,
        displayName.value,
        username.value,
        avatarUrl.value,
        termsAccepted.value,
      );

      if (res != null) {
        // Biyard.info("Success to signup user");
        Get.rootDelegate.offNamed(AppRoutes.selectTopicScreen);
      } else {
        Biyard.error(
          "Failed to signup",
          "Signup failed. Please try again later.",
        );
      }
    } finally {}
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.signupScreen);
  }

  @override
  void onClose() {
    displayNameController.removeListener(_syncDisplayName);
    usernameController.removeListener(_syncUsername);
    displayNameController.dispose();
    usernameController.dispose();
    super.onClose();
  }
}
