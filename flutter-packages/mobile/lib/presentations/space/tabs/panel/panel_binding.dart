import 'package:ratel/exports.dart';

class PanelBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PanelController>(() => PanelController());
  }
}
