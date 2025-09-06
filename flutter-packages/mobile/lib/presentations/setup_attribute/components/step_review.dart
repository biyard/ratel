import 'package:ratel/exports.dart';

class StepReview extends StatelessWidget {
  const StepReview({
    super.key,
    required this.name,
    required this.birth,
    required this.nationality,
    required this.expire,
    required this.gender,
    required this.onRecapture,
    required this.onDone,
  });

  final String name;
  final String birth;
  final String nationality;
  final String expire;
  final String gender;
  final VoidCallback onRecapture;
  final VoidCallback onDone;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                6.vgap,
                label('Name'),
                field(name),
                12.vgap,
                label('Birth date'),
                field(birth),
                12.vgap,
                label('Nationality'),
                field(nationality),
                12.vgap,
                label('Expiration date'),
                field(expire),
                12.vgap,
                label('Gender'),
                field(gender),
              ],
            ),
          ),
          Spacer(),
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
              Flexible(
                fit: FlexFit.tight,
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
    );
  }

  Padding label(String t) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 3),
      child: Text(
        t,
        style: const TextStyle(
          color: AppColors.neutral400,
          fontSize: 11,
          height: 1.45,
          fontWeight: FontWeight.w400,
        ),
      ),
    );
  }

  AppTextField field(String v) {
    return AppTextField(
      hint: v,
      controller: TextEditingController(text: v),
      onChanged: (_) {},
      keyboardType: TextInputType.text,
      obscureText: false,
      readOnly: true,
    );
  }
}
