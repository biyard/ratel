import 'dart:math' as math;

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

    final isForward = index > currentIndex;
    setState(() {
      currentIndex = index;
    });

    Navigator.of(context).pushReplacement(
      PageRouteBuilder(
        pageBuilder: (_, __, ___) => page,
        transitionsBuilder: (_, animation, __, child) {
          final begin = Offset(isForward ? 1.0 : -1.0, 0.0);
          const end = Offset.zero;
          const curve = Curves.easeInOut;

          final tween = Tween(
            begin: begin,
            end: end,
          ).chain(CurveTween(curve: curve));

          return SlideTransition(
            position: animation.drive(tween),
            child: child,
          );
        },
        transitionDuration: const Duration(milliseconds: 300),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final sheetWidth = 330.0;

    final rawInset = MediaQuery.of(context).viewPadding.bottom;
    final inset = math.min(rawInset, 8.0);

    const double navContentHeight = 60.0;
    final double barHeight = navContentHeight + rawInset + 10;

    return Scaffold(
      extendBody: true,
      bottomNavigationBar: buildBottomNav(barHeight, inset),
      body: Stack(
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
          // Positioned(
          //   left: 0,
          //   right: 0,
          //   bottom: 0,
          //   child: buildBottomNav(barHeight, inset),
          // ),
        ],
      ),
    );
  }

  AnimatedBuilder buildBottomNav(double barHeight, double inset) {
    return AnimatedBuilder(
      animation: _panelCtrl,
      builder: (_, child) {
        final dy = barHeight * _panelCtrl.value;
        return Transform.translate(offset: Offset(0, dy), child: child);
      },
      child: SizedBox(
        height: barHeight,
        child: LayoutBuilder(
          builder: (context, constraints) {
            final itemWidth = constraints.maxWidth / routes.length;
            return Stack(
              children: [
                Container(
                  decoration: const BoxDecoration(
                    color: AppColors.neutral800,
                    border: Border(
                      top: BorderSide(color: AppColors.iconPrimary, width: 0.1),
                    ),
                  ),
                ),
                Padding(
                  padding: EdgeInsets.only(bottom: inset),
                  child: BottomNavigationBar(
                    type: BottomNavigationBarType.fixed,
                    backgroundColor: Colors.transparent,
                    currentIndex: currentIndex,
                    selectedItemColor: AppColors.primary,
                    unselectedItemColor: AppColors.neutral500,
                    selectedLabelStyle: const TextStyle(
                      fontSize: 12,
                      fontWeight: FontWeight.w500,
                      color: AppColors.primary,
                    ),
                    unselectedLabelStyle: const TextStyle(
                      fontSize: 12,
                      fontWeight: FontWeight.w500,
                      color: AppColors.neutral500,
                    ),
                    onTap: onTap,
                    items: [
                      BottomNavigationBarItem(
                        icon: SvgPicture.asset(
                          Assets.home,
                          width: 25,
                          height: 25,
                          colorFilter: const ColorFilter.mode(
                            AppColors.iconPrimary,
                            BlendMode.srcIn,
                          ),
                        ),
                        activeIcon: SvgPicture.asset(
                          Assets.home,
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
                          Assets.chat,
                          width: 25,
                          height: 25,
                          colorFilter: const ColorFilter.mode(
                            AppColors.iconPrimary,
                            BlendMode.srcIn,
                          ),
                        ),
                        activeIcon: SvgPicture.asset(
                          Assets.chat,
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
                          Assets.plus,
                          width: 28,
                          height: 28,
                          colorFilter: const ColorFilter.mode(
                            AppColors.iconPrimary,
                            BlendMode.srcIn,
                          ),
                        ),
                        activeIcon: SvgPicture.asset(
                          Assets.chat,
                          width: 28,
                          height: 28,
                          colorFilter: const ColorFilter.mode(
                            AppColors.primary,
                            BlendMode.srcIn,
                          ),
                        ),
                        label: "",
                      ),
                      BottomNavigationBarItem(
                        icon: SvgPicture.asset(
                          Assets.notification,
                          width: 25,
                          height: 25,
                          colorFilter: const ColorFilter.mode(
                            AppColors.iconPrimary,
                            BlendMode.srcIn,
                          ),
                        ),
                        activeIcon: SvgPicture.asset(
                          Assets.notification,
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
                          Assets.notification,
                          width: 25,
                          height: 25,
                          colorFilter: const ColorFilter.mode(
                            AppColors.iconPrimary,
                            BlendMode.srcIn,
                          ),
                        ),
                        activeIcon: SvgPicture.asset(
                          Assets.notification,
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
                Positioned(
                  top: 0,
                  left: 0,
                  right: 0,
                  child: Row(
                    children: List.generate(
                      routes.length,
                      (i) => SizedBox(
                        width: itemWidth,
                        child: Container(
                          height: 1 / MediaQuery.of(context).devicePixelRatio,
                          color: i == currentIndex
                              ? Colors.transparent
                              : AppColors.iconPrimary,
                        ),
                      ),
                    ),
                  ),
                ),
                Positioned(
                  top: 0,
                  left: itemWidth * currentIndex,
                  child: Container(
                    width: itemWidth,
                    height: 2,
                    color: AppColors.primary,
                  ),
                ),
              ],
            );
          },
        ),
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
