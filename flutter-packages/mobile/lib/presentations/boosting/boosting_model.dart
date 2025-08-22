class BoostingModel {
  final int id;
  final int updatedAt;
  final int points;
  final int ratels;
  final bool exchanged;

  const BoostingModel({
    required this.id,
    required this.updatedAt,
    required this.points,
    required this.ratels,
    required this.exchanged,
  });
}

class RewardModel {
  final int points;

  const RewardModel({required this.points});
}
