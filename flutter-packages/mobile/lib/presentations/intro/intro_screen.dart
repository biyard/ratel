import 'package:ratel/exports.dart';
import 'package:ratel/presentations/intro/intro_controller.dart';

class IntroScreen extends GetWidget<IntroController> {
  const IntroScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<IntroController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Intro", style: TextStyle(color: Colors.blue))],
      ),
    );
  }
}
