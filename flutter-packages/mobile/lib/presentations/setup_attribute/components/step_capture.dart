import 'package:ratel/exports.dart';

class StepCapture extends StatelessWidget {
  const StepCapture({
    super.key,
    required this.imageUrl,
    required this.onCapture,
  });

  final String imageUrl;
  final VoidCallback onCapture;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Column(
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
              Container(
                width: double.infinity,
                height: 220,
                decoration: BoxDecoration(
                  color: const Color(0xFF2E2D37),
                  borderRadius: BorderRadius.circular(6),
                ),
                clipBehavior: Clip.antiAlias,
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
            ],
          ),
          const Spacer(),
          //FIXME: remove this widget when passport is implemented
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
          24.vgap,
        ],
      ),
    );
  }
}
