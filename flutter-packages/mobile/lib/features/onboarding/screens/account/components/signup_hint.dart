import 'package:ratel/exports.dart';

class SignupHint extends StatelessWidget {
  final VoidCallback onSignup;

  const SignupHint({super.key, required this.onSignup});

  @override
  Widget build(BuildContext context) {
    return RichText(
      text: TextSpan(
        style: const TextStyle(
          fontSize: 15,
          color: Color(0xFFD4D4D4),
          fontWeight: FontWeight.w400,
          height: 22 / 15,
        ),
        children: [
          const TextSpan(text: "Don't have an account?  "),
          WidgetSpan(
            alignment: PlaceholderAlignment.middle,
            child: GestureDetector(
              onTap: onSignup,
              child: const Text(
                'Sign up',
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w400,
                  color: Color(0xFFFCB300),
                  height: 22 / 15,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
