import 'package:ratel/exports.dart';

class SpacesBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SpacesController>(() => SpacesController());
  }
}
