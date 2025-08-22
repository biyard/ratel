import 'package:ratel/exports.dart';

enum SetupAttrStep { info, countryCheck, capture, review }

class SetupAttributeController extends BaseController {
  final step = SetupAttrStep.info.obs;

  //FIXME: fix to using passport library
  final selectedCountry = 'Country'.obs;
  final capturedPath = ''.obs;

  final name = 'Lee Chanhui'.obs;
  final birth = '1999-01-12'.obs;
  final nationality = 'Republic of Korea'.obs;
  final expire = '2034-08-06'.obs;
  final gender = 'Male'.obs;

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

  void mockCapture() {
    capturedPath.value =
        'https://images.unsplash.com/photo-1543852786-1cf6624b9987?w=1200';
    step.value = SetupAttrStep.review;
  }

  void recapture() => step.value = SetupAttrStep.capture;

  void done() => Get.rootDelegate.offNamed(AppRoutes.welcomeScreen);
}
