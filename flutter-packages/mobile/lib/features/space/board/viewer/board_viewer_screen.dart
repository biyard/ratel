import 'package:ratel/exports.dart';

class BoardViewerScreen extends GetWidget<BoardViewerController> {
  const BoardViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardViewerController>(
      scrollable: false,
      child: Text("Space Board Viewer Screen"),
    );
  }
}
