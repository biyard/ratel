import 'package:ratel/exports.dart';

class CreatePostBinding extends Bindings {
  @override
  void dependencies() {
    final args =
        Get.rootDelegate.arguments() as Map<String, dynamic>? ?? const {};
    final postPk = args['postPk'] as String?;

    Get.put(CreatePostController(postPk: postPk));
  }
}
