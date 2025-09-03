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
          onParsed: (info) async {
            controller.name.value = displayName(info.firstName, info.lastName);
            controller.birth.value = fmtYmd(info.birthDate);
            controller.expire.value = fmtYmd(info.expirationDate);
            controller.gender.value = info.gender;
            controller.nationality.value = mapNationality(info.nationality);

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

            controller.next();
          },
        );
      case VerifiedStep.review:
        return StepReview(
          name: controller.name.value,
          birth: controller.birth.value,
          nationality: controller.nationality.value,
          expire: controller.expire.value,
          gender: controller.gender.value,
          onRecapture: controller.back,
          onDone: controller.goMain,
        );
    }
  }
}
