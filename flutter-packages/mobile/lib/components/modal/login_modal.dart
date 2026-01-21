import 'package:ratel/exports.dart';
import 'package:ratel/localization/login_localization.dart';

class LoginModal extends StatelessWidget {
  LoginModal({super.key});

  final authService = Get.find<AuthService>();

  @override
  Widget build(BuildContext context) {
    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
      backgroundColor: AppColors.bg,
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 400),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 30, vertical: 25),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              ModalTitle(text: LoginLocalization.joinMovement),
              35.vgap,
              LoginButton(
                onPressed: () async {
                  // await authService.connectToGoogle(Config.redirectUrl);
                },
                label: LoginLocalization.continueWithGoogle,
                icon: Assets.googleImage,
              ),
              35.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Text(
                    LoginLocalization.privacyPolicy,
                    style: AppFonts.modalPolicyTextStyle,
                  ),
                  10.gap,
                  Text(
                    LoginLocalization.termsOfService,
                    style: AppFonts.modalPolicyTextStyle,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class LoginButton extends StatelessWidget {
  final VoidCallback onPressed;
  final Widget? icon;
  final String label;

  const LoginButton({
    super.key,
    required this.onPressed,
    this.icon,
    required this.label,
  });

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      style: ElevatedButton.styleFrom(
        backgroundColor: AppColors.black,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
        padding: const EdgeInsets.symmetric(vertical: 22, horizontal: 20),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          if (icon != null) ...[icon!, 20.gap],
          Text(label, style: AppFonts.modalDescriptionTextStyle),
        ],
      ),
    );
  }
}
