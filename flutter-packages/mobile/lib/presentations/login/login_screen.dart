import 'package:ratel/exports.dart';

class LoginScreen extends GetWidget<LoginController> {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<LoginController>(child: Text("Login"));
  }
}
