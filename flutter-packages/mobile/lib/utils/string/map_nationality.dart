String mapNationality(String codeOrName) {
  final m = {
    'ROK': 'Republic of Korea',
    'KOR': 'Republic of Korea',
    'USA': 'United States',
    'GBR': 'United Kingdom',
    'JPN': 'Japan',
    'CHN': 'China',
    'CAN': 'Canada',
    'AUS': 'Australia',
    'DEU': 'Germany',
    'FRA': 'France',
    'ESP': 'Spain',
    'ITA': 'Italy',
    'NGA': 'Nigeria',
  };
  return m[codeOrName.toUpperCase()] ?? codeOrName;
}
