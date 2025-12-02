import 'package:ratel/exports.dart';

class PollCreatorScreen extends GetWidget<PollCreatorController> {
  const PollCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PollCreatorController>(
      scrollable: false,
      child: Text("Space Poll Creator Screen"),
    );
  }
}
