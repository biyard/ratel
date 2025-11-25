import 'package:ratel/exports.dart';

export 'screens/intro/intro_binding.dart';
export 'screens/intro/intro_controller.dart';
export 'screens/intro/intro_model.dart';
export 'screens/intro/intro_screen.dart';
export 'screens/login/login_binding.dart';
export 'screens/login/login_controller.dart';
export 'screens/login/login_screen.dart';
export 'screens/welcome/welcome_binding.dart';
export 'screens/welcome/welcome_controller.dart';
export 'screens/welcome/welcome_screen.dart';

const String welcomeScreen = '/welcome';
const String loginScreen = '/login';
const String introScreen = '/intro';

List<GetPage> onboardingPages = [
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
    name: welcomeScreen,
    page: () => const WelcomeScreen(),
    binding: WelcomeBinding(),
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
];
