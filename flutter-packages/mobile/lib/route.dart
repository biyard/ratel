import 'package:ratel/exports.dart';

class AppRoutes {
  static const String mainScreen = '/';
  static const String explore = "/explore";
  static const String home = "/home";
  static const String myNetwork = "/network";
  static const String message = "/message";

  static const String notification = "/notification";

  static List<GetPage> pages = [
    GetPage(
      name: mainScreen,
      page: () => MainScreen(),
      binding: MainBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: explore,
      page: () => ExploreScreen(),
      binding: ExploreBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: home,
      page: () => HomeScreen(),
      binding: HomeBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: myNetwork,
      page: () => NetworkScreen(),
      binding: NetworkBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: message,
      page: () => MessageScreen(),
      binding: MessageBinding(),
      transition: Transition.noTransition,
    ),

    GetPage(
      name: notification,
      page: () => const NotificationScreen(),
      transition: Transition.noTransition,
    ),
  ];
}
