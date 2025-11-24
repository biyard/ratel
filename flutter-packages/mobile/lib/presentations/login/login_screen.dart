import 'package:ratel/exports.dart';
import 'package:ratel/presentations/login/component/social_chooser.dart';
import 'package:ratel/presentations/login/component/email_login_form.dart';

class LoginScreen extends GetWidget<LoginController> {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<LoginController>(
      scrollable: false,
      child: SizedBox.expand(
        child: Obx(
          () => AnimatedSwitcher(
            duration: const Duration(milliseconds: 260),
            switchInCurve: Curves.easeOut,
            switchOutCurve: Curves.easeIn,
            transitionBuilder: (child, anim) {
              final slide = Tween<Offset>(
                begin: const Offset(0, 0.12),
                end: Offset.zero,
              ).animate(anim);
              final fade = Tween<double>(begin: 0, end: 1).animate(anim);
              return SlideTransition(
                position: slide,
                child: FadeTransition(opacity: fade, child: child),
              );
            },
            child: controller.showEmailForm.value
                ? const EmailLoginForm(key: ValueKey('emailForm'))
                : const SocialChooser(key: ValueKey('chooser')),
          ),
        ),
      ),
    );
  }
}
