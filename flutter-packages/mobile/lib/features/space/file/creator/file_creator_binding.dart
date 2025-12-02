import 'package:ratel/exports.dart';

class FileCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<FileCreatorController>(() => FileCreatorController());
  }
}
