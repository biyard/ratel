import 'package:ratel/exports.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int currentIndex = 0;

  final routes = [
    AppRoutes.home,
    AppRoutes.myNetwork,
    AppRoutes.mySpaces,
    AppRoutes.notification,
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
      bottomNavigationBar: Container(
        decoration: const BoxDecoration(
          border: Border(
            top: BorderSide(color: AppColors.iconPrimary, width: 0.1),
          ),
        ),
        child: BottomNavigationBar(
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
              icon: SvgPicture.asset(
                Assets.home,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(
                  AppColors.iconPrimary,
                  BlendMode.srcIn,
                ),
              ),
              activeIcon: SvgPicture.asset(
                Assets.home,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
              ),
              label: 'Home',
            ),
            BottomNavigationBarItem(
              icon: SvgPicture.asset(
                Assets.people,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(
                  AppColors.iconPrimary,
                  BlendMode.srcIn,
                ),
              ),
              activeIcon: SvgPicture.asset(
                Assets.people,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
              ),
              label: 'My network',
            ),
            BottomNavigationBarItem(
              icon: SvgPicture.asset(
                Assets.chat,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(
                  AppColors.iconPrimary,
                  BlendMode.srcIn,
                ),
              ),
              activeIcon: SvgPicture.asset(
                Assets.chat,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
              ),
              label: 'My spaces',
            ),
            BottomNavigationBarItem(
              icon: SvgPicture.asset(
                Assets.notification,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(
                  AppColors.iconPrimary,
                  BlendMode.srcIn,
                ),
              ),
              activeIcon: SvgPicture.asset(
                Assets.notification,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
              ),
              label: 'Notification',
            ),
            BottomNavigationBarItem(
              icon: SvgPicture.asset(
                Assets.mail,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(
                  AppColors.iconPrimary,
                  BlendMode.srcIn,
                ),
              ),
              activeIcon: SvgPicture.asset(
                Assets.mail,
                width: 25,
                height: 25,
                colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
              ),
              label: 'Messages',
            ),
          ],
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
        return const MySpacesScreen();
      case AppRoutes.notification:
        return const NotificationScreen();
      default:
        return const MessageScreen();
    }
  }
}
