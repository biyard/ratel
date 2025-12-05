import 'package:ratel/exports.dart';

class SettingScreen extends GetWidget<SettingController> {
  const SettingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SettingController>(
      scrollable: false,
      child: Text("Space Setting Screen"),
    );
  }
}
