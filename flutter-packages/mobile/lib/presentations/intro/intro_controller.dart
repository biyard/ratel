import 'package:ratel/exports.dart';

class IntroController extends BaseController {
  final pageController = PageController();
  final currentPage = 0.obs;

  final pages = const [
    OnboardingData(
      title1: 'Purpose-Driven',
      title2: 'Social Media',
      description:
          'Connect with a community that acts with \nintention.\nTurn ideas into impact through verified\nparticipation and transparent decision-making.',
    ),
    OnboardingData(
      title1: 'Activate',
      title2: 'Your Projects',
      description:
          'Go beyond updates—use posts and spaces to \ninvite your audience to collaborate, contribute, \nand shape your project’s success together.',
    ),
    OnboardingData(
      title1: 'Enable Discussion',
      title2: 'with AI Agent',
      description:
          'Let our AI Agent moderate conversations, \nsummarize key points, and guide discussions\ntoward meaningful, actionable outcomes.',
    ),
  ];

  void onPageChanged(int i) => currentPage.value = i;

  Future<void> next() async {
    if (currentPage.value < pages.length - 1) {
      await pageController.nextPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeOut,
      );
    } else {
      await autoLogin();
      Get.rootDelegate.offNamed(AppRoutes.loginScreen);
    }
  }

  Future<void> autoLogin() async {
    final auth = AuthApi();
    await auth.init();
    final ok = await auth.tryAutoSignIn();
    if (ok) {
      Get.rootDelegate.offNamed(AppRoutes.mainScreen);
    }
  }

  @override
  void onClose() {
    pageController.dispose();
    super.onClose();
  }
}
