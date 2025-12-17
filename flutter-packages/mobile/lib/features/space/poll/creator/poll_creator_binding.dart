import 'package:ratel/exports.dart';

class PollCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollCreatorController>(() => PollCreatorController());
  }
}
