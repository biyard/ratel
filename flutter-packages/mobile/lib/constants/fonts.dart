import 'package:ratel/exports.dart';

abstract class AppFonts {
  static const textTheme = TextTheme(
    displayLarge: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 96.0,
      color: AppColors.textPrimaryColor,
    ),
    displayMedium: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 60.0,
      color: AppColors.textPrimaryColor,
    ),
    displaySmall: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 48.0,
      color: AppColors.textPrimaryColor,
    ),
    headlineLarge: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 40.0,
      color: AppColors.textPrimaryColor,
    ),
    headlineMedium: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 34.0,
      color: AppColors.textPrimaryColor,
    ),
    headlineSmall: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 24.0,
      color: AppColors.textPrimaryColor,
    ),
    titleLarge: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 20.0,
      color: AppColors.textPrimaryColor,
    ),
    titleMedium: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 16.0,
      color: AppColors.textPrimaryColor,
    ),
    titleSmall: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.bold,
      fontSize: 12.0,
      color: AppColors.textPrimaryColor,
    ),
    bodyMedium: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.normal,
      fontSize: 14.0,
      color: AppColors.textPrimaryColor,
    ),
    labelLarge: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.normal,
      fontSize: 12.0,
      color: AppColors.textPrimaryColor,
    ),
    labelSmall: TextStyle(
      fontFamily: "RalewayNoDigits",
      fontFamilyFallback: ["Pretendard"],
      fontStyle: FontStyle.normal,
      fontWeight: FontWeight.normal,
      fontSize: 10.0,
      color: AppColors.textPrimaryColor,
    ),
  );
  static const mainSlogan = TextStyle(
    fontFamily: "RalewayNoDigits",
    fontFamilyFallback: ["Pretendard"],
    fontStyle: FontStyle.normal,
    fontWeight: FontWeight.w400,
    fontSize: 72.0,
    color: AppColors.textPrimaryColor,
    shadows: [
      Shadow(blurRadius: 12.0, color: AppColors.keyColor, offset: Offset(0, 0)),
    ],
  );

  static const mainTextStyle = TextStyle(
    fontFamily: 'Poppins',
    fontSize: 16,
    fontWeight: FontWeight.w400,
    color: AppColors.textPrimaryColor,
  );

  static const modalHeaderTextStyle = TextStyle(
    fontFamily: "RalewayNoDigits",
    fontFamilyFallback: ["Pretendard"],
    fontSize: 20,
    fontWeight: FontWeight.w700,
    color: Colors.white,
    height: 20 / 20,
  );

  static const modalDescriptionTextStyle = TextStyle(
    fontFamily: "RalewayNoDigits",
    fontFamilyFallback: ["Pretendard"],
    color: Colors.white,
    fontSize: 16,
    fontWeight: FontWeight.w600,
    height: 16 / 16,
  );

  static const modalPolicyTextStyle = TextStyle(
    fontFamily: "RalewayNoDigits",
    fontFamilyFallback: ["Pretendard"],
    color: AppColors.neutral400,
    fontSize: 12,
    fontWeight: FontWeight.w500,
    height: 12 / 12,
  );
}
