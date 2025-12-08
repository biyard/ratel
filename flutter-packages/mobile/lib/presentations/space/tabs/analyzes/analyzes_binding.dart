import 'package:ratel/exports.dart';

class AnalyzesBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<AnalyzesController>(() => AnalyzesController());
    Get.lazyPut<AnalyzesCreatorController>(() => AnalyzesCreatorController());
  }
}
