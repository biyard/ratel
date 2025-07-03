import 'package:ratel/exports.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int currentIndex = 1;

  final routes = [
    AppRoutes.explore,
    AppRoutes.home,
    AppRoutes.myNetwork,
    AppRoutes.message,
  ];

  void onTap(int index) {
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
    return Scaffold(
      extendBody: true,
      body: SafeArea(
        child: Stack(
          children: [
            Column(
              children: [
                const Header(),
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
          ],
        ),
      ),
      bottomNavigationBar: BottomNavigationBar(
        type: BottomNavigationBarType.fixed,
        backgroundColor: AppColors.neutral800,
        currentIndex: currentIndex,
        selectedItemColor: Colors.white,
        unselectedItemColor: AppColors.neutral500,
        selectedLabelStyle: const TextStyle(
          fontSize: 12,
          fontWeight: FontWeight.w500,
          color: Colors.white,
        ),
        unselectedLabelStyle: const TextStyle(
          fontSize: 12,
          fontWeight: FontWeight.w500,
          color: AppColors.neutral500,
        ),
        onTap: onTap,
        items: [
          BottomNavigationBarItem(
            icon: Assets.internetImage,
            activeIcon: Assets.internetActiveImage,
            label: 'Explore',
          ),
          BottomNavigationBarItem(
            icon: Assets.home1Image,
            activeIcon: Assets.home1ActiveImage,
            label: 'Home',
          ),
          BottomNavigationBarItem(
            icon: Assets.userGroupImage,
            activeIcon: Assets.userGroupActiveImage,
            label: 'My network',
          ),
          BottomNavigationBarItem(
            icon: Assets.roundBubbleImage,
            activeIcon: Assets.roundBubbleActiveImage,
            label: 'Message',
          ),
        ],
      ),
    );
  }

  Widget _routeToPage(String? name) {
    switch (name) {
      case AppRoutes.explore:
        return const ExploreScreen();
      case AppRoutes.home:
        return const HomeScreen();
      case AppRoutes.myNetwork:
        return const NetworkScreen();
      default:
        return const MessageScreen();
    }
  }
}
