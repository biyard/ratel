import 'package:ratel/exports.dart';

class BoardCreatorScreen extends GetWidget<BoardCreatorController> {
  const BoardCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardCreatorController>(
      scrollable: false,
      child: Text("Space Board Creator Screen"),
    );
  }
}
