import 'package:ratel/exports.dart';

class DraftScreen extends GetWidget<DraftController> {
  const DraftScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<DraftController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Draft", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
