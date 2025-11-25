import 'package:ratel/exports.dart';

class SocialChooser extends GetWidget<LoginController> {
  const SocialChooser({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        SizedBox(
          width: double.infinity,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              24.vgap,
              Image.asset(Assets.logoSquare, width: 150, fit: BoxFit.contain),
            ],
          ),
        ),
        50.vgap,
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                "Sign in to\nyour account",
                style: AppFonts.textTheme.headlineMedium?.copyWith(
                  color: Colors.white,
                  fontSize: 34,
                  fontWeight: FontWeight.w900,
                  height: 1.1,
                ),
              ),
              30.vgap,
              _SocialButton(
                label: "Continue with Apple",
                background: Colors.white,
                foreground: AppColors.neutral900,
                borderColor: Colors.transparent,
                leading: SvgPicture.asset(Assets.apple, width: 24, height: 24),
                onTap: controller.signInWithApple,
                trailing: const Icon(Icons.chevron_right, size: 20),
              ),
              12.vgap,
              _SocialButton(
                label: "Continue with Google",
                background: AppColors.neutral900,
                foreground: Colors.white,
                borderColor: AppColors.borderPrimary,
                leading: SvgPicture.asset(Assets.google, width: 24, height: 24),
                onTap: controller.signInWithGoogle,
                trailing: const Icon(Icons.chevron_right, size: 20),
              ),
              12.vgap,
              _SocialButton(
                label: "Continue with email",
                background: AppColors.neutral900,
                foreground: Colors.white,
                borderColor: AppColors.borderPrimary,
                leading: SvgPicture.asset(Assets.email, width: 24, height: 24),
                onTap: controller.openEmailForm,
                trailing: const Icon(Icons.chevron_right, size: 20),
              ),
            ],
          ),
        ),
        30.vgap,
        Column(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  "Don't have an account?  ",
                  style: AppFonts.textTheme.bodyMedium?.copyWith(
                    color: const Color(0xffd4d4d4),
                    fontSize: 15,
                    fontWeight: FontWeight.w400,
                    height: 1.2,
                  ),
                ),
                GestureDetector(
                  onTap: controller.goToSignup,
                  child: Text(
                    "Sign up",
                    style: AppFonts.textTheme.bodyMedium?.copyWith(
                      color: AppColors.primary,
                      fontSize: 15,
                      fontWeight: FontWeight.w400,
                      height: 1.2,
                    ),
                  ),
                ),
              ],
            ),
            28.vgap,
          ],
        ),
      ],
    );
  }
}

class _SocialButton extends StatelessWidget {
  final String label;
  final Color background;
  final Color foreground;
  final Color borderColor;
  final SvgPicture leading;
  final Widget trailing;
  final VoidCallback onTap;

  const _SocialButton({
    required this.label,
    required this.background,
    required this.foreground,
    required this.borderColor,
    required this.leading,
    required this.trailing,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(100),
      onTap: onTap,
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.symmetric(horizontal: 30, vertical: 17),
        decoration: BoxDecoration(
          color: background,
          borderRadius: BorderRadius.circular(100),
          border: Border.all(color: borderColor, width: 1),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            leading,
            10.gap,
            Expanded(
              child: Text(
                label,
                style: AppFonts.textTheme.bodyMedium?.copyWith(
                  color: foreground,
                  fontSize: 15,
                  fontWeight: FontWeight.w600,
                  height: 1.2,
                ),
              ),
            ),
            IconTheme(
              data: IconThemeData(color: foreground.withValues(alpha: 0.7)),
              child: trailing,
            ),
          ],
        ),
      ),
    );
  }
}
