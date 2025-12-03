import 'package:ratel/exports.dart';

class BoardScreen extends GetWidget<BoardController> {
  const BoardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<BoardController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const BoardCreatorScreen()
          : const BoardViewerScreen(),
    );
  }
}
