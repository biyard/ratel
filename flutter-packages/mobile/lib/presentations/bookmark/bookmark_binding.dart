import 'package:ratel/exports.dart';

class BookmarkBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BookmarkController>(() => BookmarkController());
  }
}
