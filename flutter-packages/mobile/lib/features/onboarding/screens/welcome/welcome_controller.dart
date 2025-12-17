import 'package:ratel/exports.dart';

class WelcomeController extends BaseController {
  void goNext() {
    Get.rootDelegate.offAndToNamed(AppRoutes.mainScreen);
  }
}
