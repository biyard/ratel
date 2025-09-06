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
      scrollable: false,
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
                  onParsed: (info) async {
                    controller.name.value = displayName(
                      info.firstName,
                      info.lastName,
                    );
                    controller.birth.value = fmtYmd(info.birthDate);
                    controller.expire.value = fmtYmd(info.expirationDate);
                    controller.gender.value = info.gender;
                    controller.nationality.value = mapNationality(
                      info.nationality,
                    );
                    controller.selectedCountry.value =
                        controller.nationality.value;

                    final store = SecurePassportStore();
                    await store.saveFromPassport(controller.userId.value, info);

                    final hasBirth = await store.s.containsKey(
                      key: 'passport_birth_date ${controller.userId.value}',
                      aOptions: SecurePassportStore.aOpts,
                      iOptions: SecurePassportStore.iOpts,
                    );
                    final all = await store.s.readAll(
                      aOptions: SecurePassportStore.aOpts,
                      iOptions: SecurePassportStore.iOpts,
                    );
                    logger.d('contains birth=$hasBirth, all=$all');

                    controller.toReview();
                  },
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
