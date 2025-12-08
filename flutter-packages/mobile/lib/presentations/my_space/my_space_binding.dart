import 'package:ratel/exports.dart';

class MySpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<MySpaceController>(() => MySpaceController());
  }
}
