import 'package:ratel/exports.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen>
    with SingleTickerProviderStateMixin {
  int currentIndex = 0;
  late final AnimationController _panelCtrl;
  late final MainController controller;

  @override
  void initState() {
    super.initState();
    controller = Get.find<MainController>();
    _panelCtrl = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
    );
  }

  @override
  void dispose() {
    _panelCtrl.dispose();
    super.dispose();
  }

  final routes = [
    AppRoutes.home,
    AppRoutes.mySpaces,
    '__plus__',
    AppRoutes.notification,
    AppRoutes.myPage,
  ];

  Future<void> openSidePanel() async {
    if (Get.isBottomSheetOpen ?? false) {
      Get.back();
      await Future.delayed(const Duration(milliseconds: 80));
    }
    _panelCtrl.forward();
  }

  void closeSidePanel() {
    _panelCtrl.reverse();
  }

  void onTap(int index) async {
    if (index == 2) {
      await controller.createPost();
      return;
    }

    if (index == currentIndex) return;

    final routeName = routes[index];
    final page = _routeToPage(routeName);
    final context = Get.nestedKey(1)!.currentContext!;

    setState(() {
      currentIndex = index;
    });

    Navigator.of(context).pushReplacement(
      PageRouteBuilder(
        pageBuilder: (_, __, ___) => page,
        transitionsBuilder: (_, __, ___, child) => child,
        transitionDuration: Duration.zero,
        reverseTransitionDuration: Duration.zero,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final sheetWidth = 330.0;

    const double navContentHeight = 60.0;
    final double barHeight = navContentHeight;

    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: const SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: Brightness.light,
        systemNavigationBarColor: Colors.transparent,
        systemNavigationBarIconBrightness: Brightness.light,
        systemNavigationBarContrastEnforced: false,
      ),
      child: Scaffold(
        backgroundColor: Colors.transparent,
        extendBody: true,
        bottomNavigationBar: buildBottomNav(barHeight),
        body: Container(
          color: AppColors.bg,
          child: Stack(
            children: [
              Column(
                children: [
                  Expanded(
                    child: Navigator(
                      key: Get.nestedKey(1),
                      initialRoute: AppRoutes.home,
                      onGenerateRoute: (settings) => GetPageRoute(
                        page: () => _routeToPage(settings.name),
                        settings: settings,
                      ),
                    ),
                  ),
                ],
              ),
              Builder(
                builder: (context) {
                  final topInset = MediaQuery.of(context).padding.top;
                  return Positioned(
                    top: 0,
                    left: 0,
                    right: 0,
                    height: topInset,
                    // status bar
                    child: IgnorePointer(
                      child: Container(
                        decoration: BoxDecoration(
                          gradient: LinearGradient(
                            begin: Alignment.topCenter,
                            end: Alignment.bottomCenter,
                            colors: [
                              AppColors.bg.withAlpha(150),
                              Colors.transparent,
                            ],
                          ),
                        ),
                      ),
                    ),
                  );
                },
              ),
              AnimatedBuilder(
                animation: _panelCtrl,
                builder: (_, __) {
                  final show = _panelCtrl.value > 0.0;
                  return IgnorePointer(
                    ignoring: !show,
                    child: Opacity(
                      opacity: 0.6 * _panelCtrl.value,
                      child: GestureDetector(
                        onTap: closeSidePanel,
                        child: Container(color: Colors.black),
                      ),
                    ),
                  );
                },
              ),
              AnimatedBuilder(
                animation: _panelCtrl,
                builder: (_, __) {
                  final dx = -sheetWidth * (1.0 - _panelCtrl.value);
                  return Transform.translate(
                    offset: Offset(dx, 0),
                    child: Align(
                      alignment: Alignment.centerLeft,
                      child: Obx(
                        () => SidePanel(
                          width: sheetWidth,
                          user: controller.user.value,
                          onClose: closeSidePanel,
                        ),
                      ),
                    ),
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }

  AnimatedBuilder buildBottomNav(double navContentHeight) {
    return AnimatedBuilder(
      animation: _panelCtrl,
      builder: (_, child) {
        final dy = navContentHeight * _panelCtrl.value;
        return Transform.translate(offset: Offset(0, dy), child: child);
      },
      child: Builder(
        builder: (context) {
          final bottomInset = MediaQuery.of(context).viewPadding.bottom;
          final totalHeight = navContentHeight + bottomInset;

          return SizedBox(
            height: totalHeight,
            child: Stack(
              children: [
                Container(
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      begin: Alignment.bottomCenter,
                      end: Alignment.topCenter,
                      colors: [
                        AppColors.neutral900,
                        AppColors.neutral900.withAlpha((255 * 0.9).toInt()),
                      ],
                    ),
                    border: Border(
                      top: BorderSide(color: AppColors.iconPrimary, width: 0.1),
                    ),
                  ),
                ),
                Positioned(
                  left: 0,
                  right: 0,
                  bottom: bottomInset,
                  child: SizedBox(
                    height: navContentHeight,
                    child: LayoutBuilder(
                      builder: (context, constraints) {
                        final itemWidth = constraints.maxWidth / routes.length;

                        return Stack(
                          children: [
                            MediaQuery.removePadding(
                              context: context,
                              removeBottom: true,
                              child: Theme(
                                data: Theme.of(context).copyWith(
                                  splashFactory: NoSplash.splashFactory,
                                  highlightColor: Colors.transparent,
                                  splashColor: Colors.transparent,
                                  hoverColor: Colors.transparent,
                                  bottomNavigationBarTheme:
                                      const BottomNavigationBarThemeData(
                                        elevation: 0,
                                        backgroundColor: Colors.transparent,
                                      ),
                                  shadowColor: Colors.transparent,
                                  colorScheme: Theme.of(context).colorScheme
                                      .copyWith(
                                        surfaceTint: Colors.transparent,
                                      ),
                                ),
                                child: BottomNavigationBar(
                                  type: BottomNavigationBarType.fixed,
                                  backgroundColor: Colors.transparent,
                                  currentIndex: currentIndex,
                                  selectedItemColor: AppColors.primary,
                                  unselectedItemColor: AppColors.neutral500,
                                  showSelectedLabels: false,
                                  showUnselectedLabels: false,
                                  onTap: onTap,
                                  items: [
                                    BottomNavigationBarItem(
                                      icon: SvgPicture.asset(
                                        Assets.bottomHome,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          Colors.white,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      activeIcon: SvgPicture.asset(
                                        Assets.bottomHome,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          AppColors.primary,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      label: MainLocalization.home,
                                    ),
                                    BottomNavigationBarItem(
                                      icon: SvgPicture.asset(
                                        Assets.bottomPalace,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          Colors.white,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      activeIcon: SvgPicture.asset(
                                        Assets.bottomPalace,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          AppColors.primary,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      label: MainLocalization.spaces,
                                    ),
                                    BottomNavigationBarItem(
                                      icon: SvgPicture.asset(
                                        Assets.bottomCreate,
                                        width: 25,
                                        height: 25,
                                      ),
                                      activeIcon: SvgPicture.asset(
                                        Assets.bottomCreate,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          AppColors.primary,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      label: "",
                                    ),
                                    BottomNavigationBarItem(
                                      icon: SvgPicture.asset(
                                        Assets.bottomNotification,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          Colors.white,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      activeIcon: SvgPicture.asset(
                                        Assets.bottomNotification,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          AppColors.primary,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      label: MainLocalization.notification,
                                    ),
                                    BottomNavigationBarItem(
                                      icon: SvgPicture.asset(
                                        Assets.bottomUser,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          Colors.white,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      activeIcon: SvgPicture.asset(
                                        Assets.bottomUser,
                                        width: 25,
                                        height: 25,
                                        colorFilter: const ColorFilter.mode(
                                          AppColors.primary,
                                          BlendMode.srcIn,
                                        ),
                                      ),
                                      label: MainLocalization.my,
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          ],
                        );
                      },
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _routeToPage(String? name) {
    switch (name) {
      case AppRoutes.home:
        return const HomeScreen();
      case AppRoutes.myNetwork:
        return const NetworkScreen();
      case AppRoutes.mySpaces:
        return const MySpaceScreen();
      case AppRoutes.notification:
        return const NotificationScreen();
      case AppRoutes.myPage:
        return const MyPageScreen();
      default:
        return const MessageScreen();
    }
  }
}
