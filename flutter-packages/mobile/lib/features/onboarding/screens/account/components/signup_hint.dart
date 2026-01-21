import 'package:ratel/exports.dart';

class SignupHint extends StatelessWidget {
  final VoidCallback onSignup;

  const SignupHint({super.key, required this.onSignup});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 22,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(
            "Don’t have an account?",
            style: AppFonts.textTheme.bodyMedium?.copyWith(
              color: AppColors.neutral300,
              fontWeight: FontWeight.w400,
              fontSize: 15,
              height: 22 / 15,
            ),
          ),
          10.gap,
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onSignup,
            child: Text(
              'Sign up',
              style: AppFonts.textTheme.bodyMedium?.copyWith(
                color: AppColors.primary,
                fontWeight: FontWeight.w400,
                fontSize: 15,
                height: 22 / 15,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
