enum SpacePublishState { draft, published }

SpacePublishState spacePublishStateFromJson(dynamic v) {
  if (v == null) return SpacePublishState.draft;

  final s = v.toString().toLowerCase();
  switch (s) {
    case 'published':
      return SpacePublishState.published;
    case 'draft':
    default:
      return SpacePublishState.draft;
  }
}

String spacePublishStateToJson(SpacePublishState v) {
  switch (v) {
    case SpacePublishState.draft:
      return 'draft';
    case SpacePublishState.published:
      return 'published';
  }
}
