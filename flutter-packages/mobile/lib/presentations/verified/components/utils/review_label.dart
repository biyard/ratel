import 'package:ratel/exports.dart';

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
