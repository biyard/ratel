import 'package:ratel/exports.dart';

class MainBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<MainController>(() => MainController());
    Get.lazyPut<ExploreController>(() => ExploreController());
    Get.lazyPut<HomeController>(() => HomeController());
    Get.lazyPut<MessageController>(() => MessageController());
    Get.lazyPut<NetworkController>(() => NetworkController());
    Get.lazyPut<NotificationController>(() => NotificationController());
    Get.lazyPut<SpacesController>(() => SpacesController());
  }
}
