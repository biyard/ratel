import 'dart:convert';
import 'package:flutter/services.dart' show rootBundle;

class CountryCode {
  final String code;
  final String name;
  final String dialCode;

  const CountryCode({
    required this.code,
    required this.name,
    required this.dialCode,
  });

  factory CountryCode.fromJson(Map<String, dynamic> json) {
    return CountryCode(
      code: json['code'] as String,
      name: json['name'] as String,
      dialCode: (json['dial_code'] as String).replaceAll('+', ''),
    );
  }
}

const kDefaultCountryCode = CountryCode(
  code: 'KR',
  name: 'Republic of Korea',
  dialCode: '82',
);

class CountryCodes {
  static List<CountryCode>? _cache;

  static Future<List<CountryCode>> load() async {
    if (_cache != null) return _cache!;
    final raw = await rootBundle.loadString('assets/json/country_codes.json');
    final List list = jsonDecode(raw) as List;
    _cache = list
        .map((e) => CountryCode.fromJson(e as Map<String, dynamic>))
        .toList();
    return _cache!;
  }
}
