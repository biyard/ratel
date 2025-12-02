import 'package:ratel/exports.dart';

class PollsScreen extends GetWidget<PollsController> {
  const PollsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PollsController>(
      scrollable: false,
      child: Text("Space Polls Screen"),
    );
  }
}
