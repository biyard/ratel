import 'package:ratel/exports.dart';

enum SetupAttrStep { info, countryCheck, capture, review }

class SetupAttributeController extends BaseController {
  Rx<int> userId = 0.obs;
  final userApi = Get.find<UserApi>();
  final step = SetupAttrStep.info.obs;

  final selectedCountry = 'Country'.obs;
  final capturedPath = ''.obs;

  final name = ''.obs;
  final birth = ''.obs;
  final nationality = ''.obs;
  final expire = ''.obs;
  final gender = ''.obs;

  @override
  void onInit() {
    super.onInit();
    getUser();
  }

  void getUser() async {
    final item = await userApi.getUserInfo();
    userId(item.id);
  }

  void goBack() {
    switch (step.value) {
      case SetupAttrStep.info:
        Get.rootDelegate.offNamed(AppRoutes.connectionScreen);
        break;
      case SetupAttrStep.countryCheck:
        step.value = SetupAttrStep.info;
        break;
      case SetupAttrStep.capture:
        step.value = SetupAttrStep.countryCheck;
        break;
      case SetupAttrStep.review:
        step.value = SetupAttrStep.capture;
        break;
    }
  }

  void skip() => Get.rootDelegate.offNamed(AppRoutes.welcomeScreen);

  void toCountryCheck() => step.value = SetupAttrStep.countryCheck;

  void toCapture() => step.value = SetupAttrStep.capture;

  void toReview() {
    step.value = SetupAttrStep.review;
  }

  void recapture() => step.value = SetupAttrStep.capture;

  void done() => Get.rootDelegate.offNamed(AppRoutes.welcomeScreen);
}
