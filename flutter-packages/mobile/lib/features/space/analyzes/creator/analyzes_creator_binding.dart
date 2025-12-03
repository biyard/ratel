import 'package:ratel/exports.dart';

class AnalyzesCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<AnalyzesCreatorController>(() => AnalyzesCreatorController());
  }
}
