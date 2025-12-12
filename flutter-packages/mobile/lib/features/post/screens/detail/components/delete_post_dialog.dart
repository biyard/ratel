import 'package:ratel/exports.dart';

class DeletePostDialog extends StatelessWidget {
  const DeletePostDialog({super.key});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: AppColors.bg,
      surfaceTintColor: AppColors.bg,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20.0)),
      content: FittedBox(
        fit: BoxFit.cover,
        child: SizedBox(
          width: 350,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Delete this post?',
                style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                  fontSize: 24,
                  height: 32 / 24,
                ),
              ),
              24.vgap,
              const Text(
                'This action cannot be undone.',
                textAlign: TextAlign.center,
                style: TextStyle(
                  color: AppColors.neutral300,
                  fontWeight: FontWeight.w400,
                  fontSize: 12,
                  height: 22 / 15,
                ),
              ),
              35.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  InkWell(
                    onTap: () => Navigator.pop(context, false),
                    child: RoundContainer(
                      width: 95,
                      height: 50,
                      color: Colors.transparent,
                      radius: 10,
                      child: const Padding(
                        padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                        child: Text(
                          'Cancel',
                          style: TextStyle(
                            color: AppColors.neutral300,
                            fontWeight: FontWeight.w700,
                            fontSize: 16,
                          ),
                        ),
                      ),
                    ),
                  ),
                  10.gap,
                  InkWell(
                    onTap: () => Navigator.pop(context, true),
                    child: RoundContainer(
                      width: 206,
                      height: 50,
                      color: AppColors.primary,
                      radius: 10,
                      child: const Center(
                        child: Padding(
                          padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                          child: Text(
                            'Delete',
                            style: TextStyle(
                              color: AppColors.bg,
                              fontWeight: FontWeight.w700,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
