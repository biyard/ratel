import 'package:ratel/exports.dart';

class DetailPostBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<DetailPostController>(() => DetailPostController());
  }
}
