enum SpaceVisibility { private, public, team, teamGroupMember }

SpaceVisibility spaceVisibilityFromJson(dynamic v) {
  if (v == null) return SpaceVisibility.private;

  final s = v.toString().toLowerCase();

  if (s == 'public') return SpaceVisibility.public;
  if (s.startsWith('team_group_member')) {
    return SpaceVisibility.teamGroupMember;
  }
  if (s.startsWith('team')) {
    return SpaceVisibility.team;
  }
  return SpaceVisibility.private;
}

String spaceVisibilityToJson(SpaceVisibility v) {
  switch (v) {
    case SpaceVisibility.private:
      return 'private';
    case SpaceVisibility.public:
      return 'public';
    case SpaceVisibility.team:
      return 'team';
    case SpaceVisibility.teamGroupMember:
      return 'team_group_member';
  }
}
