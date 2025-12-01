import 'package:ratel/exports.dart';

class FileBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<FileController>(() => FileController());
  }
}
