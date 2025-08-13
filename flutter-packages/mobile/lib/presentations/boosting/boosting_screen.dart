import 'package:ratel/exports.dart';

class BoostingScreen extends GetWidget<BoostingController> {
  const BoostingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<BoostingController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Boosting", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
