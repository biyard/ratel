enum TeamGroupPermission {
  postRead(0),
  postWrite(1),
  postEdit(2),
  postDelete(3),

  spaceRead(10),
  spaceWrite(11),
  spaceEdit(12),
  spaceDelete(13),

  teamAdmin(20),
  teamEdit(21),
  groupEdit(22),

  managePromotions(62),
  manageNews(63);

  final int bit;
  const TeamGroupPermission(this.bit);
}

class TeamGroupPermissions {
  final int raw;
  final Set<TeamGroupPermission> _set;

  TeamGroupPermissions._(this.raw, this._set);

  factory TeamGroupPermissions.fromInt(int value) {
    final set = <TeamGroupPermission>{};
    for (final perm in TeamGroupPermission.values) {
      final mask = 1 << perm.bit;
      if ((value & mask) != 0) {
        set.add(perm);
      }
    }
    return TeamGroupPermissions._(value, set);
  }

  factory TeamGroupPermissions.fromJson(dynamic v) {
    if (v is int) {
      return TeamGroupPermissions.fromInt(v);
    }
    if (v is String) {
      final s = v.trim().toLowerCase();
      if (s.isEmpty || s == 'null') {
        return TeamGroupPermissions.fromInt(0);
      }
      final parsed = int.tryParse(s);
      if (parsed != null) {
        return TeamGroupPermissions.fromInt(parsed);
      }
    }
    return TeamGroupPermissions.fromInt(0);
  }

  bool has(TeamGroupPermission permission) => _set.contains(permission);

  bool get isAdmin => has(TeamGroupPermission.teamAdmin);
}
