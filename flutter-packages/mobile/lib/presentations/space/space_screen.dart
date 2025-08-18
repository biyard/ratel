import 'package:ratel/exports.dart';

class SpaceScreen extends GetWidget<SpaceController> {
  const SpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SpaceController>(
      child: Text("space screen", style: TextStyle(color: Colors.white)),
    );
  }
}
