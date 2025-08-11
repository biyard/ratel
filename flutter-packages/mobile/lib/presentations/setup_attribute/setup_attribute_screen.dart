import 'package:ratel/exports.dart';
import 'package:ratel/presentations/setup_attribute/components/step_capture.dart';
import 'package:ratel/presentations/setup_attribute/components/step_country.dart';
import 'package:ratel/presentations/setup_attribute/components/step_info.dart';
import 'package:ratel/presentations/setup_attribute/components/step_review.dart';

class SetupAttributeScreen extends GetWidget<SetupAttributeController> {
  const SetupAttributeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SetupAttributeController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Obx(
          () => Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                height: 70,
                child: Row(
                  children: [
                    InkWell(onTap: controller.goBack, child: Assets.backIcon),
                    10.gap,
                    const Text(
                      'Set up attribute',
                      style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                      ),
                    ),
                  ],
                ),
              ),
              const Text(
                'Passport',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 36,
                  fontWeight: FontWeight.w900,
                  height: 1.22,
                ),
              ),
              30.vgap,
              if (controller.step.value == SetupAttrStep.info)
                StepInfo(
                  onSkip: controller.skip,
                  onNext: controller.toCountryCheck,
                )
              else if (controller.step.value == SetupAttrStep.countryCheck)
                StepCountry(onNo: controller.skip, onYes: controller.toCapture)
              else if (controller.step.value == SetupAttrStep.capture)
                StepCapture(
                  imageUrl: controller.capturedPath.value,
                  onCapture: controller.mockCapture,
                )
              else
                StepReview(
                  name: controller.name.value,
                  birth: controller.birth.value,
                  nationality: controller.nationality.value,
                  expire: controller.expire.value,
                  gender: controller.gender.value,
                  onRecapture: controller.recapture,
                  onDone: controller.done,
                ),
            ],
          ),
        ),
      ),
    );
  }
}
