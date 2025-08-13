import 'package:ratel/exports.dart';

enum VerifiedStep { myCredential, info, countryCheck, capture, review }

class VerifiedController extends BaseController {
  final Rx<VerifiedStep> step = VerifiedStep.myCredential.obs;
  final RxList<VerifiedModel> credentials = <VerifiedModel>[
    VerifiedModel(
      label: "Blood type",
      value: "A-",
      metadata:
          "https://metadata.ratel.foundation/d98d2d3a632d2272e6f6ec729feb85745f547322.jpg",
    ),
    VerifiedModel(
      label: "Gender",
      value: "Main",
      metadata:
          "https://metadata.ratel.foundation/46cac616c26546e62a9ce3ea614d47f7ce5e2369.jpg",
    ),
    VerifiedModel(
      label: "Residential Area",
      value: "Busan",
      metadata:
          "https://metadata.ratel.foundation/8ea8c023f59aaa5d4e974cd7f95fd1c73882d15b.jpg",
    ),
    VerifiedModel(
      label: "Annual Salary",
      value: "\$20k",
      metadata:
          "https://metadata.ratel.foundation/e03aefdfdc27a9457f40abc575c7b49145004a56.jpg",
    ),
  ].obs;

  void next() {
    switch (step.value) {
      case VerifiedStep.myCredential:
        step.value = VerifiedStep.info;
        break;
      case VerifiedStep.info:
        step.value = VerifiedStep.countryCheck;
        break;
      case VerifiedStep.countryCheck:
        step.value = VerifiedStep.capture;
        break;
      case VerifiedStep.capture:
        step.value = VerifiedStep.review;
        break;
      case VerifiedStep.review:
        break;
    }
  }

  void goMain() {
    step.value = VerifiedStep.myCredential;
  }

  void back() {
    switch (step.value) {
      case VerifiedStep.review:
        step.value = VerifiedStep.capture;
        break;
      case VerifiedStep.capture:
        step.value = VerifiedStep.countryCheck;
        break;
      case VerifiedStep.countryCheck:
        step.value = VerifiedStep.info;
        break;
      case VerifiedStep.info:
        step.value = VerifiedStep.myCredential;
        break;
      case VerifiedStep.myCredential:
        Get.rootDelegate.offNamed(AppRoutes.mainScreen);
        break;
    }
  }
}
