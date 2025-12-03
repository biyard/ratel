typedef Json = Map<String, dynamic>;

class SpaceRequirementModel {
  final String relatedPk;
  final String relatedSk;
  final int order;
  final SpaceRequirementType type;
  final bool responded;

  const SpaceRequirementModel({
    required this.relatedPk,
    required this.relatedSk,
    required this.order,
    required this.type,
    required this.responded,
  });

  factory SpaceRequirementModel.fromJson(Json j) {
    return SpaceRequirementModel(
      relatedPk: (j['related_pk'] ?? '') as String,
      relatedSk: (j['related_sk'] ?? '') as String,
      order: (j['order'] ?? 0) as int,
      type: spaceRequirementTypeFromJson(j['typ']),
      responded: (j['responded'] ?? false) as bool,
    );
  }

  Json toJson() {
    return {
      'related_pk': relatedPk,
      'related_sk': relatedSk,
      'order': order,
      'typ': spaceRequirementTypeToJson(type),
      'responded': responded,
    };
  }
}

enum SpaceRequirementType { none, prePoll }

SpaceRequirementType spaceRequirementTypeFromJson(dynamic v) {
  if (v == null) return SpaceRequirementType.none;

  final s = v.toString().toLowerCase();
  switch (s) {
    case 'pre_poll':
    case 'prepoll':
    case 'prepoll ':
    case 'pre poll':
      return SpaceRequirementType.prePoll;
    case 'none':
    default:
      return SpaceRequirementType.none;
  }
}

String spaceRequirementTypeToJson(SpaceRequirementType t) {
  switch (t) {
    case SpaceRequirementType.none:
      return 'none';
    case SpaceRequirementType.prePoll:
      return 'pre_poll';
  }
}
