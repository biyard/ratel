import 'package:ratel/exports.dart';

class SpaceScreen extends GetWidget<SpaceController> {
  const SpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SpaceController>(
      scrollable: false,
      child: Text("Space Screen"),
    );
  }
}
