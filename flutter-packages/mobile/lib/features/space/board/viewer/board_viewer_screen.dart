import 'package:ratel/exports.dart';

class BoardViewerScreen extends GetWidget<BoardViewerController> {
  const BoardViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // TODO: implement the UI for Board Viewer Screen
    return Layout<BoardViewerController>(
      scrollable: false,
      child: Text("Space Board Viewer Screen"),
    );
  }
}
