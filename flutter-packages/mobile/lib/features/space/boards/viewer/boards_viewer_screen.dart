import 'package:ratel/exports.dart';

class BoardsViewerScreen extends GetWidget<BoardsViewerController> {
  const BoardsViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardsViewerController>(
      scrollable: false,
      child: Text("Boards Viewer Screen"),
    );
  }
}
