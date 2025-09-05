import 'package:ratel/exports.dart';
import 'package:ratel/presentations/verified/components/utils/review_field.dart';
import 'package:ratel/presentations/verified/components/utils/review_label.dart';

class StepMedicalReview extends StatelessWidget {
  const StepMedicalReview({
    super.key,
    required this.height,
    required this.weight,
    required this.bmi,
    required this.onRecapture,
    required this.onDone,
  });

  final double height;
  final double weight;
  final double bmi;
  final VoidCallback onRecapture;
  final VoidCallback onDone;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 50),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: 70,
            child: Row(
              children: [
                InkWell(
                  onTap: onRecapture,
                  child: SvgPicture.asset(Assets.back, width: 16, height: 16),
                ),
                10.gap,
                const Text(
                  'Set up attribute',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
          ),
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Padding(
                    padding: EdgeInsets.fromLTRB(4, 0, 4, 16),
                    child: Text(
                      'Verified Attributes',
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 32,
                        fontWeight: FontWeight.w800,
                        height: 1.1,
                      ),
                    ),
                  ),
                  const Text(
                    "We never save your privacy into our server.\n"
                    "It will only be utilized to create anonymous credential called SSI (self-sovereign identity).",
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontSize: 12,
                      height: 1.33,
                    ),
                  ),
                  18.vgap,
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      6.vgap,
                      label('Height (cm)'),
                      field(height.toString()),
                      12.vgap,
                      label('Weight (kg)'),
                      field(weight.toString()),
                      12.vgap,
                      label('BMI (kg/m²)'),
                      field(bmi.toString()),
                    ],
                  ),
                  24.vgap,
                  Row(
                    children: [
                      SizedBox(
                        width: 140,
                        child: TextButton(
                          onPressed: onRecapture,
                          child: const Text(
                            'Re-capture',
                            style: TextStyle(
                              color: AppColors.neutral300,
                              fontWeight: FontWeight.w700,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ),
                      10.gap,
                      Expanded(
                        child: ElevatedButton(
                          onPressed: onDone,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.primary,
                            foregroundColor: Colors.black,
                            padding: const EdgeInsets.symmetric(vertical: 14.5),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(10),
                            ),
                          ),
                          child: const Text(
                            'DONE',
                            style: TextStyle(
                              color: AppColors.bg,
                              fontSize: 16,
                              fontWeight: FontWeight.w700,
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                  24.vgap,
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
