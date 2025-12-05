import 'package:ratel/exports.dart';

class PostBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PostController>(() => PostController());
  }
}
