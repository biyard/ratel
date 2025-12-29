import 'package:ratel/exports.dart';

class AlreadyHaveAccountRow extends StatelessWidget {
  const AlreadyHaveAccountRow({super.key, required this.onTapLogin});

  final VoidCallback onTapLogin;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 22,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(
            'Already have an account?',
            style: AppFonts.textTheme.bodyMedium?.copyWith(
              color: const Color(0xFFD4D4D4),
              fontWeight: FontWeight.w400,
              fontSize: 15,
              height: 22 / 15,
            ),
          ),
          10.gap,
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onTapLogin,
            child: Text(
              'Log in',
              style: AppFonts.textTheme.bodyMedium?.copyWith(
                color: const Color(0xFFFCB300),
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
