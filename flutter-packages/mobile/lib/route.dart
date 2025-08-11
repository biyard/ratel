import 'package:ratel/exports.dart';

class AppRoutes {
  static const String mainScreen = '/dashboard';
  static const String introScreen = '/intro';
  static const String loginScreen = '/login';
  static const String signupScreen = '/signup';
  static const String verificationScreen = '/verification';
  static const String welcomeScreen = '/welcome';
  static const String explore = "/dashboard/explore";
  static const String home = "/dashboard/home";
  static const String myNetwork = "/dashboard/network";
  static const String message = "/dashboard/message";

  static const String notification = "/dashboard/notification";

  static List<GetPage> pages = [
    GetPage(
      name: mainScreen,
      page: () => MainScreen(),
      binding: MainBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: introScreen,
      page: () => const IntroScreen(),
      binding: IntroBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: loginScreen,
      page: () => const LoginScreen(),
      binding: LoginBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: signupScreen,
      page: () => const SignupScreen(),
      binding: SignupBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: verificationScreen,
      page: () => const VerificationScreen(),
      binding: VerificationBinding(),
      transition: Transition.noTransition,
    ),
    GetPage(
      name: welcomeScreen,
      page: () => const WelcomeScreen(),
      binding: WelcomeBinding(),
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
