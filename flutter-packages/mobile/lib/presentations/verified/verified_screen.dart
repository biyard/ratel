import 'package:ratel/exports.dart';
import 'package:ratel/presentations/verified/components/step_review.dart';
import 'package:ratel/presentations/verified/components/step_capture.dart';
import 'package:ratel/presentations/verified/components/step_country.dart';
import 'package:ratel/presentations/verified/components/step_info.dart';
import 'package:ratel/presentations/verified/components/credentials.dart';

class VerifiedScreen extends GetWidget<VerifiedController> {
  const VerifiedScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final h = MediaQuery.of(context).size.height;
    return Layout<VerifiedController>(
      scrollable: false,
      child: Obx(
        () =>
            SizedBox(height: h - 65, child: _buildStep(controller.step.value)),
      ),
    );
  }

  Widget _buildStep(VerifiedStep step) {
    switch (step) {
      case VerifiedStep.myCredential:
        return Credentials(
          credentials: controller.credentials.value,
          did: controller.didId.value,
          onNext: controller.next,
        );
      case VerifiedStep.info:
        return StepInfo(onSkip: controller.back, onNext: controller.next);
      case VerifiedStep.countryCheck:
        return StepCountry(
          onPrev: controller.back,
          onNo: controller.goMain,
          onYes: controller.next,
        );
      case VerifiedStep.capture:
        return StepCapture(
          onPrev: controller.back,
          imageUrl: "",
          onCapture: controller.next,
        );
      case VerifiedStep.review:
        return StepReview(
          birth: "1999-01-12",
          onRecapture: controller.back,
          onDone: controller.goMain,
        );
    }
  }
}
