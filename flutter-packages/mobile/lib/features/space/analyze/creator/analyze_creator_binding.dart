import 'package:ratel/exports.dart';

class AnalyzeCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<AnalyzeCreatorController>(() => AnalyzeCreatorController());
  }
}
