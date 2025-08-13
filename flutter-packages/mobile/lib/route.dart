import 'package:ratel/exports.dart';

class AppRoutes {
  static const String mainScreen = '/dashboard';
  static const String introScreen = '/intro';
  static const String loginScreen = '/login';
  static const String signupScreen = '/signup';
  static const String verificationScreen = '/verification';
  static const String welcomeScreen = '/welcome';
  static const String setupProfileScreen = '/setup-profile';
  static const String selectTopicScreen = '/select-topic';
  static const String connectionScreen = '/connection';
  static const String setupAttributeScreen = '/setup-attribute';
  static const String boostingScreen = '/boosting';
  static const String draftScreen = '/draft';
  static const String postScreen = '/post';
  static const String verifiedScreen = '/verified';
  static const String mySpaces = '/my-spaces';
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
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: boostingScreen,
      page: () => const BoostingScreen(),
      binding: BoostingBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: postScreen,
      page: () => const PostScreen(),
      binding: PostBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: draftScreen,
      page: () => const DraftScreen(),
      binding: DraftBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: verifiedScreen,
      page: () => const VerifiedScreen(),
      binding: VerifiedBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: introScreen,
      page: () => const IntroScreen(),
      binding: IntroBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: loginScreen,
      page: () => const LoginScreen(),
      binding: LoginBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: signupScreen,
      page: () => const SignupScreen(),
      binding: SignupBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: verificationScreen,
      page: () => const VerificationScreen(),
      binding: VerificationBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: setupProfileScreen,
      page: () => const SetupProfileScreen(),
      binding: SetupProfileBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: selectTopicScreen,
      page: () => const SelectTopicScreen(),
      binding: SelectTopicBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: connectionScreen,
      page: () => const ConnectionScreen(),
      binding: ConnectionBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: setupAttributeScreen,
      page: () => const SetupAttributeScreen(),
      binding: SetupAttributeBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: mySpaces,
      page: () => const MySpacesScreen(),
      binding: MySpacesBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: welcomeScreen,
      page: () => const WelcomeScreen(),
      binding: WelcomeBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: explore,
      page: () => ExploreScreen(),
      binding: ExploreBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: home,
      page: () => HomeScreen(),
      binding: HomeBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: myNetwork,
      page: () => NetworkScreen(),
      binding: NetworkBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
    GetPage(
      name: message,
      page: () => MessageScreen(),
      binding: MessageBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),

    GetPage(
      name: notification,
      page: () => const NotificationScreen(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
  ];
}

class SlideOverTransition extends CustomTransition {
  SlideOverTransition();

  @override
  Widget buildTransition(
    BuildContext context,
    Curve? curve,
    Alignment? alignment,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    Widget child,
  ) {
    final slide = Tween(
      begin: const Offset(1, 0),
      end: Offset.zero,
    ).chain(CurveTween(curve: curve ?? Curves.easeOutCubic)).animate(animation);
    return SlideTransition(position: slide, child: child);
  }
}
