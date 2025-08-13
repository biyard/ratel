import 'package:ratel/exports.dart';

class VerifiedScreen extends GetWidget<VerifiedController> {
  const VerifiedScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<VerifiedController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Verified", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
