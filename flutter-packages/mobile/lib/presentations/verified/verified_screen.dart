import 'package:ratel/exports.dart';
import 'package:ratel/presentations/verified/components/my_credentials.dart';
import 'package:ratel/presentations/verified/components/my_did_section.dart';

class VerifiedScreen extends GetWidget<VerifiedController> {
  const VerifiedScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<VerifiedController>(
      scrollable: false,
      child: Obx(() {
        final attrs = controller.attributes.value;
        return Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            MyCredentials(did: controller.didDocument.value?.id ?? ''),
            MyDidSection(controller: controller, attributes: attrs),
          ],
        );
      }),
    );
  }
}
