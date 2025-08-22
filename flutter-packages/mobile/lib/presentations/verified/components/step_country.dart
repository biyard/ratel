import 'package:ratel/exports.dart';

class StepCountry extends StatelessWidget {
  const StepCountry({
    super.key,
    required this.onPrev,
    required this.onNo,
    required this.onYes,
  });
  final VoidCallback onPrev;
  final VoidCallback onNo;
  final VoidCallback onYes;

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
                  onTap: onPrev,
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
              'Passport',
              style: TextStyle(
                color: Colors.white,
                fontSize: 32,
                fontWeight: FontWeight.w800,
                height: 1.1,
              ),
            ),
          ),

          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  "Could you observe the below icon in the front of your passport?\nOnly the passport having the below icon could be verified.",
                  style: TextStyle(
                    color: AppColors.neutral300,
                    fontSize: 12,
                    fontWeight: FontWeight.w400,
                    height: 1.33,
                  ),
                ),
                32.vgap,
                Align(
                  alignment: Alignment.center,
                  child: Container(
                    width: 192,
                    height: 267,
                    decoration: BoxDecoration(
                      color: const Color(0xFF35343F),
                      borderRadius: BorderRadius.circular(10),
                    ),
                    child: Stack(
                      children: [
                        Positioned.fill(
                          child: Align(
                            alignment: Alignment(0, -0.55),
                            child: Text(
                              'Country',
                              style: const TextStyle(
                                color: Colors.white,
                                fontSize: 20,
                                fontWeight: FontWeight.w700,
                                height: 1.25,
                              ),
                            ),
                          ),
                        ),
                        Positioned(
                          right: 18,
                          bottom: 24,
                          child: SvgPicture.asset(
                            Assets.passport,
                            width: 28,
                            height: 28,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),

          SizedBox(
            width: double.infinity,
            child: Row(
              children: [
                SizedBox(
                  width: 110,
                  child: TextButton(
                    onPressed: onNo,
                    child: const Text(
                      'NO',
                      style: TextStyle(
                        color: AppColors.neutral300,
                        fontWeight: FontWeight.w700,
                        fontSize: 16,
                      ),
                    ),
                  ),
                ),
                10.gap,
                Flexible(
                  fit: FlexFit.tight,
                  child: ElevatedButton(
                    onPressed: onYes,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primary,
                      foregroundColor: Colors.black,
                      padding: const EdgeInsets.symmetric(vertical: 14.5),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                    ),
                    child: const Text(
                      'YES',
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
}
