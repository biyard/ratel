import 'package:ratel/exports.dart';

class MySpacesBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<MySpacesController>(() => MySpacesController());
  }
}
