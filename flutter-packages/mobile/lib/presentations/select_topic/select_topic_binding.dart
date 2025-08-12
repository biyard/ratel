import 'package:ratel/exports.dart';

class SelectTopicBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SelectTopicController>(() => SelectTopicController());
  }
}
