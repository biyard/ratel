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
  final int bloodPressureSystolic;
  final int bloodPressureDiastolic;

  const MedicalInfo({
    required this.height,
    required this.weight,
    required this.bmi,
    required this.bloodPressureSystolic,
    required this.bloodPressureDiastolic,
  });

  factory MedicalInfo.fromJson(Map<String, dynamic> json) {
    final map = json['result'] is Map
        ? (json['result'] as Map).cast<String, dynamic>()
        : json;

    double toDouble(dynamic v) {
      if (v == null) return 0.0;
      if (v is num) return v.toDouble();
      if (v is String) return double.tryParse(v) ?? 0.0;
      return 0.0;
    }

    int toInt(dynamic v) {
      if (v == null) return 0;
      if (v is int) return v;
      if (v is num) return v.toInt();
      if (v is String) {
        final i = int.tryParse(v);
        if (i != null) return i;
        final d = double.tryParse(v);
        if (d != null) return d.round();
      }
      return 0;
    }

    final h = toDouble(map['height']);
    final w = toDouble(map['weight']);
    double b = toDouble(map['bmi']);
    if (b == 0.0 && h > 0 && w > 0) {
      final meters = h >= 3.0 ? h / 100.0 : h;
      b = w / (meters * meters);
    }

    final sys = toInt(map['blood_pressure_systolic']);
    final dia = toInt(map['blood_pressure_diastolic']);

    return MedicalInfo(
      height: h,
      weight: w,
      bmi: b,
      bloodPressureSystolic: sys,
      bloodPressureDiastolic: dia,
    );
  }

  Map<String, dynamic> toJson() => {
    'height': height,
    'weight': weight,
    'bmi': bmi,
    'blood_pressure_systolic': bloodPressureSystolic,
    'blood_pressure_diastolic': bloodPressureDiastolic,
  };

  MedicalInfo copyWith({
    double? height,
    double? weight,
    double? bmi,
    int? bloodPressureSystolic,
    int? bloodPressureDiastolic,
  }) {
    return MedicalInfo(
      height: height ?? this.height,
      weight: weight ?? this.weight,
      bmi: bmi ?? this.bmi,
      bloodPressureSystolic:
          bloodPressureSystolic ?? this.bloodPressureSystolic,
      bloodPressureDiastolic:
          bloodPressureDiastolic ?? this.bloodPressureDiastolic,
    );
  }

  @override
  String toString() =>
      'MedicalInfo(height: $height, weight: $weight, bmi: $bmi, '
      'bloodPressureSystolic: $bloodPressureSystolic, '
      'bloodPressureDiastolic: $bloodPressureDiastolic)';

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        other is MedicalInfo &&
            height == other.height &&
            weight == other.weight &&
            bmi == other.bmi &&
            bloodPressureSystolic == other.bloodPressureSystolic &&
            bloodPressureDiastolic == other.bloodPressureDiastolic;
  }

  @override
  int get hashCode => Object.hash(
    height,
    weight,
    bmi,
    bloodPressureSystolic,
    bloodPressureDiastolic,
  );
}
