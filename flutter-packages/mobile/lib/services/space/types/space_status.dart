enum SpaceStatus { waiting, inProgress, started, finished }

SpaceStatus spaceStatusFromJson(dynamic v) {
  if (v == null) return SpaceStatus.waiting;

  final s = v.toString().toLowerCase();

  switch (s) {
    case 'waiting':
      return SpaceStatus.waiting;
    case 'in_progress':
    case 'inprogress':
      return SpaceStatus.inProgress;
    case 'started':
      return SpaceStatus.started;
    case 'finished':
      return SpaceStatus.finished;
    default:
      return SpaceStatus.waiting;
  }
}

String spaceStatusToJson(SpaceStatus s) {
  switch (s) {
    case SpaceStatus.waiting:
      return 'waiting';
    case SpaceStatus.inProgress:
      return 'in_progress';
    case SpaceStatus.started:
      return 'started';
    case SpaceStatus.finished:
      return 'finished';
  }
}
