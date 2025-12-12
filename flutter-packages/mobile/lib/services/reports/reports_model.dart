class ReportContentResponse {
  final bool reported;

  const ReportContentResponse({required this.reported});

  factory ReportContentResponse.fromJson(Map<String, dynamic> json) {
    return ReportContentResponse(reported: json['reported'] == true);
  }
}
