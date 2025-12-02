import 'package:ratel/exports.dart';

class MemberScreen extends GetWidget<MemberController> {
  const MemberScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<MemberController>(
      scrollable: false,
      child: Text("Space Member Screen"),
    );
  }
}
