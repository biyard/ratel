import 'package:ratel/exports.dart';

class CountryPickerSheet extends StatefulWidget {
  const CountryPickerSheet({super.key});

  @override
  State<CountryPickerSheet> createState() => _CountryPickerSheetState();
}

class _CountryPickerSheetState extends State<CountryPickerSheet> {
  final TextEditingController _searchCtrl = TextEditingController();
  List<CountryCode> _all = [];
  List<CountryCode> _filtered = [];

  @override
  void initState() {
    super.initState();
    _load();
    _searchCtrl.addListener(_onSearch);
  }

  Future<void> _load() async {
    final list = await CountryCodes.load();
    setState(() {
      _all = list;
      _filtered = list;
    });
  }

  void _onSearch() {
    final q = _searchCtrl.text.toLowerCase();
    setState(() {
      _filtered = _all
          .where(
            (c) => c.name.toLowerCase().contains(q) || c.dialCode.contains(q),
          )
          .toList();
    });
  }

  @override
  void dispose() {
    _searchCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: SizedBox(
        height: MediaQuery.of(context).size.height * 0.8,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 12, 20, 24),
          child: Column(
            children: [
              Center(
                child: Container(
                  width: 50,
                  height: 4,
                  margin: const EdgeInsets.only(bottom: 16),
                  decoration: BoxDecoration(
                    color: const Color(0xFF3A3A3A),
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
              ),

              Container(
                height: 52,
                width: double.infinity,
                padding: const EdgeInsets.symmetric(horizontal: 20),
                decoration: BoxDecoration(
                  color: const Color(0xFF101010),
                  borderRadius: BorderRadius.circular(100),
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _searchCtrl,
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 16,
                          fontWeight: FontWeight.w500,
                        ),
                        decoration: const InputDecoration(
                          isCollapsed: true,
                          border: InputBorder.none,
                          hintText: 'Search',
                          hintStyle: TextStyle(
                            color: Color(0xFF525252),
                            fontSize: 16,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(width: 8),
                    SizedBox(
                      width: 24,
                      height: 24,
                      child: Center(
                        child: Icon(
                          Icons.search,
                          size: 20,
                          color: const Color(0xFF737373),
                        ),
                      ),
                    ),
                  ],
                ),
              ),

              10.vgap,

              Expanded(
                child: _filtered.isEmpty
                    ? const Center(
                        child: CircularProgressIndicator(
                          strokeWidth: 2,
                          color: Colors.white54,
                        ),
                      )
                    : ListView.separated(
                        itemCount: _filtered.length,
                        separatorBuilder: (_, __) =>
                            const Divider(color: Colors.white12, height: 1),
                        itemBuilder: (context, index) {
                          final c = _filtered[index];
                          return ListTile(
                            onTap: () => Navigator.of(context).pop(c),
                            title: Text(
                              c.name,
                              style: const TextStyle(color: Colors.white),
                            ),
                            trailing: Text(
                              '+${c.dialCode}',
                              style: const TextStyle(
                                color: AppColors.primary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                          );
                        },
                      ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
