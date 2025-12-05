import 'package:ratel/exports.dart';

class MyPageBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<MyPageController>(() => MyPageController());
  }
}
