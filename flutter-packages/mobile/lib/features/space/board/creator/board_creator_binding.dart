import 'package:ratel/exports.dart';

class BoardCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardCreatorController>(() => BoardCreatorController());
  }
}
