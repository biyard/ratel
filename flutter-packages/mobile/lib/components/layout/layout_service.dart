import 'package:ratel/exports.dart';

// add below code to main.dart
class LayoutService extends GetxService {
  static void init() {
    Get.put<LayoutService>(LayoutService());
  }

  Rx<String> selectedRoute = "/".obs;
  Rx<int> selectedIndex = 0.obs;
  Rx<int> homePageType = 0
      .obs; //0: dashboard, 1: create selection, 2: create agit, 3: create collection, 4: create nft, 5: inquiry, 6: agit detail
  Rx<bool> loadingState = false.obs;
  void updateRoute(String route) {
    selectedRoute(route);
  }

  void changeHomePageType(int type) {
    logger.d("change home page type called ${type}");
    homePageType(type);
  }

  void changeLoadingState(bool loading) {
    loadingState(loading);
  }
}

class LayoutObserver extends NavigatorObserver {
  @override
  void didPush(Route<dynamic> route, Route<dynamic>? previousRoute) {
    if (route.settings.name != null) {
      Get.find<LayoutService>().updateRoute(route.settings.name!);
    }
  }
}
