import 'package:ratel/exports.dart';

class ExploreScreen extends GetWidget<ExploreController> {
  const ExploreScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<ExploreController>(
      child: Text("Explore", style: TextStyle(color: Colors.white)),
    );
  }
}
