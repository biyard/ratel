class SpaceModel {
  final int id;
  final String title;
  final String htmlContents;
  final List<FileModel> files;
  final List<DiscussionModel> discussions;

  const SpaceModel({
    required this.id,
    required this.title,
    required this.htmlContents,
    required this.files,
    required this.discussions,
  });
}

class DiscussionModel {
  final int id;
  final int startedAt;
  final int endedAt;
  final String name;
  final String? record;

  const DiscussionModel({
    required this.id,
    required this.startedAt,
    required this.endedAt,
    required this.name,
    required this.record,
  });
}

class FileModel {
  final String name;
  final String size;
  final String ext;
  final String url;

  const FileModel({
    required this.name,
    required this.size,
    required this.ext,
    required this.url,
  });
}
