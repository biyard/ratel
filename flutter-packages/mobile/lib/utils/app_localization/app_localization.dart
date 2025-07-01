import 'package:get/get.dart';

final Map<String, String> enUs = {};
final Map<String, String> koKr = {};

String o(String key, String en) {
  if (enUs[key] != null || koKr[key] != null) throw 'duplicated key $key';
  enUs[key] = en;

  koKr[key] = en;

  return key;
}

String s(String key, String en, String ko) {
  if (enUs[key] != null || koKr[key] != null) throw 'duplicated key $key';
  enUs[key] = en;

  koKr[key] = ko;

  return key;
}

class AppLocalization extends Translations {
  @override
  Map<String, Map<String, String>> get keys => {'en_US': enUs, 'ko_KR': koKr};

  // snackbar title
  static final _errorTitle = s('errorTitle', 'Error', 'Error');
  static String get errorTitle => _errorTitle.tr;

  static final _infoTitle = s('infoTitle', 'Success', 'Success');
  static String get infoTitle => _infoTitle.tr;

  static final _warningTitle = s('warningTitle', 'Warning', 'Warning');
  static String get warningTitle => _warningTitle.tr;

  static final _referenceTitle = s('referenceTitle', 'Reference', 'Reference');
  static String get referenceTitle => _referenceTitle.tr;
}
