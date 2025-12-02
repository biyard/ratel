import 'package:ratel/exports.dart';

class AnalyzeBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<AnalyzeController>(() => AnalyzeController());
  }
}
