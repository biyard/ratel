import 'package:ratel/exports.dart';

class BoardsScreen extends GetWidget<BoardsController> {
  const BoardsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<BoardsController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const BoardsCreatorScreen()
          : const BoardsViewerScreen(),
    );
  }
}
