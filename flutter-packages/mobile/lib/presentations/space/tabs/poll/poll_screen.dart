import 'package:ratel/exports.dart';

class PollScreen extends GetWidget<PollController> {
  const PollScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PollController>(
      scrollable: false,
      child: Text("Space Poll Screen"),
    );
  }
}
