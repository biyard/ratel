import 'package:ratel/exports.dart';

class BoardScreen extends GetWidget<BoardController> {
  const BoardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoardController>(
      scrollable: false,
      child: Text("Space Board Screen"),
    );
  }
}
