import 'package:ratel/exports.dart';

class BoostingBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoostingController>(() => BoostingController());
  }
}
