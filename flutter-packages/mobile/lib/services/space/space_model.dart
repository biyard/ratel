class SpaceModel {
  final int id;
  final String title;
  final String htmlContents;
  final List<FileModel> files;

  const SpaceModel({
    required this.id,
    required this.title,
    required this.htmlContents,
    required this.files,
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
