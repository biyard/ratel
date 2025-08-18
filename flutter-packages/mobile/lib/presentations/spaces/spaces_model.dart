class SpaceSummaryModel {
  SpaceSummaryModel({
    required this.id,
    required this.createdAt,
    required this.title,
    required this.description,
    required this.imageUrl,
    required this.boostingType,
    required this.members,
  });

  final int id;
  final int createdAt;
  final String title;
  final String description;
  final String imageUrl;
  final int? boostingType; //null or 1: no boost, 2: x2, 3: x10, 4: x100
  final int members;
}
