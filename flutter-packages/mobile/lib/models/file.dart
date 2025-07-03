class File {
  final String id;
  final String name;

  File({required this.id, required this.name});

  factory File.fromJson(Map<String, dynamic> json) {
    return File(id: json['id'] as String, name: json['name'] as String);
  }
}

class FileList {
  final List<File> files;

  FileList({required this.files});

  factory FileList.fromJson(Map<String, dynamic> json) {
    return FileList(
      files: (json['files'] as List<dynamic>)
          .map((file) => File(id: file['id'], name: file['name']))
          .toList(),
    );
  }
}
