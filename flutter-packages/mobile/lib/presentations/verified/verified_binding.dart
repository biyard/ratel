import 'package:ratel/exports.dart';

class VerifiedBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<VerifiedController>(() => VerifiedController());
  }
}
