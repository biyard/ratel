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
      label: "BTC Tax",
      value: "15%",
      metadata:
          "https://metadata.ratel.foundation/2df2dc1afdf4ba79613dced79e66169b477936dd.jpg",
    ),
    VerifiedModel(
      label: "SOL Tax",
      value: "10%",
      metadata:
          "https://metadata.ratel.foundation/434a44149e8c94e2892ae2828249a8aafd928b02.jpg",
    ),
    VerifiedModel(
      label: "Blood Type",
      value: "A-",
      metadata:
          "https://metadata.ratel.foundation/565a44c913996e1c98581f5772de4dfc6f32f6be.jpg",
    ),
    VerifiedModel(
      label: "Region",
      value: "Busan",
      metadata:
          "https://metadata.ratel.foundation/1aae7a4ecfd33cfcf8bbd0a3f540b4562be19e6c.jpg",
    ),
    VerifiedModel(
      label: "Gender",
      value: "Male",
      metadata:
          "https://metadata.ratel.foundation/46cac616c26546e62a9ce3ea614d47f7ce5e2369.jpg",
    ),
    VerifiedModel(
      label: "Salary",
      value: "\$20k",
      metadata:
          "https://metadata.ratel.foundation/5f824db1c8d8c3612dcbed68021a9d9ab79e04f8.jpg",
    ),
    VerifiedModel(
      label: "Occupation",
      value: "Engineer",
      metadata:
          "https://metadata.ratel.foundation/27342ac6292fb7d2b87647841f5fab093bda09f6.jpg",
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
