import 'package:ratel/exports.dart';

enum VerifiedStep { myCredential, info, countryCheck, capture, review }

class VerifiedController extends BaseController {
  final Rx<VerifiedStep> step = VerifiedStep.myCredential.obs;
  final Rx<String> didId = 'did:ratel:q11y3sqd...'.obs;
  final RxList<VerifiedModel> credentials = <VerifiedModel>[
    VerifiedModel(
      label: "Crypto Wallet",
      value: "Active",
      metadata:
          "https://metadata.ratel.foundation/332cafb00e5a8c7011e69d364c848e514c1f17c6.jpg",
    ),
    VerifiedModel(
      label: "Birth Date",
      value: "1999-01-12",
      metadata:
          "https://metadata.ratel.foundation/565a44c913996e1c98581f5772de4dfc6f32f6be.jpg",
    ),
    VerifiedModel(
      label: "Country",
      value: "Republic of Korea",
      metadata:
          "https://metadata.ratel.foundation/1aae7a4ecfd33cfcf8bbd0a3f540b4562be19e6c.jpg",
    ),
    VerifiedModel(
      label: "Gender",
      value: "Male",
      metadata:
          "https://metadata.ratel.foundation/46cac616c26546e62a9ce3ea614d47f7ce5e2369.jpg",
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
