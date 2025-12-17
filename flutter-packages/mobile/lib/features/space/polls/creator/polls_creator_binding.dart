import 'package:ratel/exports.dart';

class PollsCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollsCreatorController>(() => PollsCreatorController());
  }
}
