import 'package:ratel/exports.dart';

class SetupAttributeBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SetupAttributeController>(() => SetupAttributeController());
  }
}
