import 'package:ratel/exports.dart';
import 'package:ratel/middlewares/intro_middleware.dart';

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
export 'screens/signup/signup_binding.dart';
export 'screens/signup/signup_controller.dart';
export 'screens/signup/signup_model.dart';
export 'screens/signup/signup_screen.dart';
export 'screens/account/account_binding.dart';
export 'screens/account/account_controller.dart';
export 'screens/account/account_screen.dart';
export 'screens/verification/verification_binding.dart';
export 'screens/verification/verification_controller.dart';
export 'screens/verification/verification_screen.dart';

const String welcomeScreen = '/welcome';
const String loginScreen = '/login';
const String introScreen = '/intro';
const String accountScreen = '/account';
const String signupScreen = '/signup';
const String verificationScreen = '/verification';

List<GetPage> onboardingPages = [
  GetPage(
    name: introScreen,
    page: () => const IntroScreen(),
    middlewares: [IntroMiddleware()],
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
  GetPage(
    name: accountScreen,
    page: () => const AccountScreen(),
    middlewares: [AccountMiddleware()],
    binding: AccountBinding(),
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
];
