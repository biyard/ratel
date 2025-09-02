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
