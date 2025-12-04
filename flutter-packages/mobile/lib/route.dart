import 'package:ratel/exports.dart';

class AppRoutes {
  static const String mainScreen = '/dashboard';
  static const String setupProfileScreen = '/setup-profile';
  static const String selectTopicScreen = '/select-topic';
  static const String connectionScreen = '/connection';
  static const String setupAttributeScreen = '/setup-attribute';
  static const String boostingScreen = '/boosting';
  static const String draftScreen = '/draft';

  static const String verifiedScreen = '/verified';
  static const String settingScreen = '/settings';
  static const String mySpaces = '/my-spaces';
  static const String explore = "/dashboard/explore";
  static const String home = "/dashboard/home";
  static const String myNetwork = "/dashboard/network";
  static const String message = "/dashboard/message";
  static const String bookmark = '/bookmark';

  static const String notification = "/dashboard/notification";
  static const String error = '/error';
  static spaceWithPk(String spacePk) => '/space/$spacePk';
  static String spacePostWithPk(String spacePk, String postPk) {
    final encSpacePk = Uri.encodeComponent(spacePk);
    final encPostPk = Uri.encodeComponent(postPk);

    return '/space/$encSpacePk/board/$encPostPk';
  }

  static deliberationSpaceWithId(int id) => '/space/$id/deliberation';
  static noticeSpaceWithId(int id) => '/space/$id/notice';
  static notFoundSpaceWithId(int id) => '/space/$id/not-found';
  static draftWithId(int id) => '/draft/$id';

  static List<GetPage> pages = [
    GetPage(
      name: '/space/:spacePk',
      page: () => const SpaceScreen(),
      binding: SpaceBinding(),
      children: [
        GetPage(
          name: '/board/:postPk',
          page: () => const BoardScreen(),
          binding: BoardBinding(),
        ),
        GetPage(
          name: '/boards',
          page: () => const BoardsScreen(),
          binding: BoardsBinding(),
        ),
        GetPage(
          name: '/file',
          page: () => const FileScreen(),
          binding: FileBinding(),
        ),
        GetPage(
          name: '/member',
          page: () => const MemberScreen(),
          binding: MemberBinding(),
        ),
        GetPage(
          name: '/overview',
          page: () => const OverviewScreen(),
          binding: OverviewBinding(),
        ),
        GetPage(
          name: '/panel',
          page: () => const PanelScreen(),
          binding: PanelBinding(),
        ),
        GetPage(
          name: '/poll',
          page: () => const PollScreen(),
          binding: PollBinding(),
        ),
        GetPage(
          name: '/polls',
          page: () => const PollsScreen(),
          binding: PollsBinding(),
        ),
        GetPage(
          name: '/analyze',
          page: () => const AnalyzeScreen(),
          binding: AnalyzeBinding(),
        ),
        GetPage(
          name: '/analyzes',
          page: () => const AnalyzesScreen(),
          binding: AnalyzesBinding(),
        ),
        GetPage(
          name: '/setting',
          page: () => const SettingScreen(),
          binding: SettingBinding(),
        ),
      ],
    ),
    GetPage(
      name: mySpaces,
      page: () => MySpaceScreen(),
      binding: MySpaceBinding(),
      customTransition: SlideOverTransition(),
      transitionDuration: const Duration(milliseconds: 300),
      opaque: true,
      curve: Curves.easeOutCubic,
    ),
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
      name: '/draft/:id',
      page: () => DraftByIdScreen(),
      binding: DraftByIdBinding(),
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
    ...onboardingPages,
    ...postPages,
    GetPage(
      name: settingScreen,
      page: () => const SettingsScreen(),
      binding: SettingsBinding(),
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
