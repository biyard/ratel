import 'package:ratel/exports.dart';

class SetupProfileBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SetupProfileController>(() => SetupProfileController());
  }
}
