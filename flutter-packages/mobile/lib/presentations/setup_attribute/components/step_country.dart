import 'package:ratel/exports.dart';

class StepCountry extends StatelessWidget {
  const StepCountry({super.key, required this.onNo, required this.onYes});
  final VoidCallback onNo;
  final VoidCallback onYes;

  @override
  Widget build(BuildContext context) {
    final h = MediaQuery.of(context).size.height;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          width: double.infinity,
          height: h - 270,
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
              50.vgap,
              Align(
                alignment: Alignment.center,
                child: Container(
                  width: 192,
                  height: 267,
                  decoration: BoxDecoration(color: const Color(0xFF35343F)),
                  child: Stack(
                    alignment: Alignment.center,
                    children: [
                      Positioned(
                        top: 50,
                        left: 57.5,
                        child: Text(
                          'Country',
                          style: TextStyle(
                            color: Colors.white,
                            fontSize: 20,
                            fontWeight: FontWeight.w700,
                            height: 1.25,
                          ),
                        ),
                      ),
                      Positioned(
                        right: 29,
                        bottom: 40,
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

        Row(
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
        24.vgap,
      ],
    );
  }
}
