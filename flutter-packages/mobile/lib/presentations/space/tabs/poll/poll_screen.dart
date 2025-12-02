import 'package:ratel/exports.dart';

class PollScreen extends GetWidget<PollController> {
  const PollScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<PollController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const PollCreatorScreen()
          : const PollViewerScreen(),
    );
  }
}
