import 'package:ratel/exports.dart';

class BoardsCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardsCreatorController>(() => BoardsCreatorController());
  }
}
