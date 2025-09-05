class PassportInfo {
  final String firstName;
  final String lastName;
  final int birthDate;
  final String nationality;
  final int expirationDate;
  final String gender;

  PassportInfo({
    required this.firstName,
    required this.lastName,
    required this.birthDate,
    required this.nationality,
    required this.expirationDate,
    required this.gender,
  });

  factory PassportInfo.fromJson(Map<String, dynamic> json) {
    final map = json['result'] is Map
        ? (json['result'] as Map).cast<String, dynamic>()
        : json;
    return PassportInfo(
      firstName: (map['first_name'] ?? '') as String,
      lastName: (map['last_name'] ?? '') as String,
      birthDate: (map['birth_date'] ?? 0) as int,
      nationality: (map['nationality'] ?? '') as String,
      expirationDate: (map['expiration_date'] ?? 0) as int,
      gender: (map['gender'] ?? map['sex'] ?? 'Other') as String,
    );
  }
}

class MedicalInfo {
  final double height;
  final double weight;
  final double bmi;

  const MedicalInfo({
    required this.height,
    required this.weight,
    required this.bmi,
  });

  factory MedicalInfo.fromJson(Map<String, dynamic> json) {
    final map = json['result'] is Map
        ? (json['result'] as Map).cast<String, dynamic>()
        : json;

    double _toDouble(dynamic v) {
      if (v == null) return 0.0;
      if (v is num) return v.toDouble();
      if (v is String) return double.tryParse(v) ?? 0.0;
      return 0.0;
    }

    final h = _toDouble(map['height']);
    final w = _toDouble(map['weight']);
    double b = _toDouble(map['bmi']);
    if (b == 0.0 && h > 0 && w > 0) {
      final meters = h >= 3.0 ? h / 100.0 : h;
      b = w / (meters * meters);
    }

    return MedicalInfo(height: h, weight: w, bmi: b);
  }

  Map<String, dynamic> toJson() => {
    'height': height,
    'weight': weight,
    'bmi': bmi,
  };

  MedicalInfo copyWith({double? height, double? weight, double? bmi}) {
    return MedicalInfo(
      height: height ?? this.height,
      weight: weight ?? this.weight,
      bmi: bmi ?? this.bmi,
    );
  }

  @override
  String toString() =>
      'MedicalResponse(height: $height, weight: $weight, bmi: $bmi)';

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        other is MedicalInfo &&
            height == other.height &&
            weight == other.weight &&
            bmi == other.bmi;
  }

  @override
  int get hashCode => Object.hash(height, weight, bmi);
}
