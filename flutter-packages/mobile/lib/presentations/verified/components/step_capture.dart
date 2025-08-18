import 'package:ratel/exports.dart';

class StepCapture extends StatelessWidget {
  const StepCapture({
    super.key,
    required this.onPrev,
    required this.imageUrl,
    required this.onCapture,
  });

  final String imageUrl;
  final VoidCallback onPrev;
  final VoidCallback onCapture;

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

          SizedBox(
            width: double.infinity,
            height: MediaQuery.of(context).size.height - 330,
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
                16.vgap,
                ClipRRect(
                  borderRadius: BorderRadius.circular(6),
                  child: Container(
                    width: double.infinity,
                    height: 220,
                    color: const Color(0xFF2E2D37),
                    child: imageUrl.isEmpty
                        ? const Center(
                            child: Icon(
                              Icons.photo_camera,
                              color: AppColors.neutral500,
                              size: 40,
                            ),
                          )
                        : Image.network(imageUrl, fit: BoxFit.cover),
                  ),
                ),
              ],
            ),
          ),

          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: onCapture,
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
          const SizedBox(height: 24),
        ],
      ),
    );
  }
}
