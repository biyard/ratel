import 'package:ratel/exports.dart';

class SignupLogo extends StatelessWidget {
  const SignupLogo({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 100,
      height: 100,
      child: Center(
        child: SvgPicture.asset(Assets.setupLogo, width: 100, height: 100),
      ),
    );
  }
}
