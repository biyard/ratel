import 'package:ratel/exports.dart';

class SignupScreen extends GetWidget<SignupController> {
  const SignupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SignupController>(
      child: SizedBox(
        width: double.infinity,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [Text("signup", style: TextStyle(color: Colors.white))],
        ),
      ),
    );
  }
}
