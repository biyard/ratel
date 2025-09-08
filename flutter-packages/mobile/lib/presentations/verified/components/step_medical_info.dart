import 'package:ratel/exports.dart';

class StepMedicalInfo extends StatelessWidget {
  const StepMedicalInfo({
    super.key,
    required this.onSkip,
    required this.onNext,
  });
  final VoidCallback onSkip;
  final VoidCallback onNext;

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
                  onTap: onSkip,
                  child: SvgPicture.asset(Assets.back, width: 16, height: 16),
                ),
                10.gap,
                const Text(
                  'Set up attribute',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(4, 0, 4, 16),
            child: Text(
              'Prepare your Medical Check-up Certificate',
              style: TextStyle(
                color: Colors.white,
                fontSize: 32,
                fontWeight: FontWeight.w800,
                height: 1.1,
              ),
            ),
          ),

          Expanded(
            child: SizedBox(
              width: double.infinity,
              height: MediaQuery.of(context).size.height - 330,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    "Could you check if the below icon is visible on your Medical Check-up Certificate?\n\nCertificates must clearly display the certificate number, hospital name, and your name/date of birth to be verified.",
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontSize: 12,
                      fontWeight: FontWeight.w400,
                      height: 1.33,
                    ),
                  ),
                  30.vgap,
                  SizedBox(
                    width: double.infinity,
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        Container(
                          width: 192,
                          decoration: BoxDecoration(
                            color: const Color(0xFF35343F),
                            borderRadius: BorderRadius.circular(4),
                          ),
                          padding: const EdgeInsets.fromLTRB(14, 18, 14, 18),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              medicalLabel("Certificate Number", "00000000"),
                              14.vgap,

                              medicalLabel(
                                "Partient Name",
                                "Jane Doh\nYYYY-MM-DD",
                              ),
                              60.vgap,

                              medicalLabel("Issue Date", "YYYY-MM-DD"),
                              16.vgap,

                              medicalLabel(
                                "Issuing Hospital",
                                "Medical Clinic",
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),

          SizedBox(
            width: double.infinity,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                SizedBox(
                  width: 110,
                  child: TextButton(
                    onPressed: onSkip,
                    child: const Padding(
                      padding: EdgeInsets.symmetric(vertical: 14.5),
                      child: Text(
                        'No',
                        style: TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.w700,
                          fontSize: 16,
                        ),
                      ),
                    ),
                  ),
                ),
                Flexible(
                  fit: FlexFit.tight,
                  child: ElevatedButton(
                    onPressed: onNext,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primary,
                      foregroundColor: Colors.black,
                      padding: const EdgeInsets.symmetric(vertical: 14.5),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                    ),
                    child: const Text(
                      'NEXT',
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
          ),
          const SizedBox(height: 24),
        ],
      ),
    );
  }

  Widget medicalLabel(String label, String value) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: TextStyle(
            color: Colors.white,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        3.vgap,
        Text(
          value,
          style: TextStyle(
            color: AppColors.neutral500,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
      ],
    );
  }
}
