import 'package:ratel/exports.dart';

class BoardsCreatorScreen extends GetWidget<BoardsCreatorController> {
  const BoardsCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardsCreatorController>(
      scrollable: false,
      child: Text("Boards Creator Screen"),
    );
  }
}
