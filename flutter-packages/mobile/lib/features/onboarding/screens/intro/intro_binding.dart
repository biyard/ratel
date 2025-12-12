import 'package:ratel/exports.dart';

class IntroBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<IntroController>(() => IntroController());
  }
}
