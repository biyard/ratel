import 'package:ratel/exports.dart';

class PollsScreen extends GetWidget<PollsController> {
  const PollsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<PollsController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const PollsCreatorScreen()
          : const PollsViewerScreen(),
    );
  }
}
