use chrono::Datelike;
use super::super::{views::format_points, *};
use crate::common::services::PointTransactionResponse;

translate! {
    BarChartTranslate;

    day: {
        en: "Day",
        ko: "일",
    },

    week: {
        en: "Week",
        ko: "주",
    },

    month_label: {
        en: "Month",
        ko: "월",
    },
}

#[derive(Debug, Clone, PartialEq)]
enum ChartPeriod {
    Day,
    Week,
    Month,
}

struct ChartBar {
    label: String,
    points: i64,
}

#[component]
pub fn PointsBarChart(
    transactions: Vec<PointTransactionResponse>,
    month: String,
) -> Element {
    let tr: BarChartTranslate = use_translate();
    let mut period = use_signal(|| ChartPeriod::Week);

    let year: i32 = month.get(..4).and_then(|s| s.parse().ok()).unwrap_or(2025);
    let mon: u32 = month.get(5..7).and_then(|s| s.parse().ok()).unwrap_or(1);

    let last_day = if mon == 12 {
        chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        chrono::NaiveDate::from_ymd_opt(year, mon + 1, 1)
    }
    .unwrap_or_default()
        - chrono::Duration::days(1);
    let days_in_month = last_day.day();

    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month_name = month_names.get((mon - 1) as usize).unwrap_or(&"");
    let date_range = format!("{} 1-{}", month_name, days_in_month);

    // Filter to award/positive transactions only
    let award_txs: Vec<&PointTransactionResponse> = transactions
        .iter()
        .filter(|tx| tx.transaction_type.eq_ignore_ascii_case("award") || tx.amount > 0)
        .collect();

    let bars = match *period.read() {
        ChartPeriod::Day => aggregate_by_day(&award_txs, year, mon, days_in_month),
        ChartPeriod::Week => aggregate_by_week(&award_txs, year, mon, days_in_month),
        ChartPeriod::Month => aggregate_by_month(&award_txs, &month),
    };

    let max_points = bars.iter().map(|b| b.points).max().unwrap_or(1).max(1);

    let current_period = period.read().clone();

    rsx! {
        div { class: "border-t border-card-border py-10",
            div { class: "flex flex-col gap-3 items-center w-full",
                // Period selector tabs
                div { class: "relative w-full",
                    div { class: "border border-white/10 rounded-[10px] h-[39px] flex items-center",
                        button {
                            class: format!(
                                "flex-1 text-center text-sm font-semibold tracking-[-0.07px] cursor-pointer {}",
                                if current_period == ChartPeriod::Day {
                                    "text-primary"
                                } else {
                                    "text-foreground-muted"
                                },
                            ),
                            onclick: move |_| period.set(ChartPeriod::Day),
                            "{tr.day}"
                        }
                        div { class: "w-px h-5 bg-white/10" }
                        button {
                            class: format!(
                                "flex-1 text-center text-sm font-semibold tracking-[-0.07px] cursor-pointer {}",
                                if current_period == ChartPeriod::Week {
                                    "text-primary"
                                } else {
                                    "text-foreground-muted"
                                },
                            ),
                            onclick: move |_| period.set(ChartPeriod::Week),
                            "{tr.week}"
                        }
                        div { class: "w-px h-5 bg-white/10" }
                        button {
                            class: format!(
                                "flex-1 text-center text-sm font-semibold tracking-[-0.07px] cursor-pointer {}",
                                if current_period == ChartPeriod::Month {
                                    "text-primary"
                                } else {
                                    "text-foreground-muted"
                                },
                            ),
                            onclick: move |_| period.set(ChartPeriod::Month),
                            "{tr.month_label}"
                        }
                    }
                }

                // Date range label
                span { class: "text-[15px] font-semibold text-text-primary tracking-[-0.075px]",
                    "{date_range}"
                }

                // Chart area
                div { class: "flex gap-[45px] items-end w-full mt-2",
                    // Y-axis labels
                    div { class: "flex flex-col gap-12 items-end text-sm font-normal text-foreground-muted opacity-80 w-20 shrink-0",
                        span { "100%" }
                        span { "60%" }
                        span { "20%" }
                    }

                    // Bars
                    div { class: "flex items-end justify-between flex-1",
                        for bar in bars.iter() {
                            {render_bar(bar, max_points)}
                        }
                    }
                }

                // X-axis labels
                div { class: "flex items-end justify-between w-full pl-[125px] pr-4 text-sm font-normal text-foreground-muted opacity-80",
                    for bar in bars.iter() {
                        span { class: "w-[50px] text-center", "{bar.label}" }
                    }
                }
            }
        }
    }
}

fn render_bar(bar: &ChartBar, max_points: i64) -> Element {
    let height_pct = if max_points > 0 {
        ((bar.points as f64 / max_points as f64) * 100.0).max(2.0)
    } else {
        2.0
    };
    let max_height = 168.0;
    let bar_height = (height_pct / 100.0 * max_height).round();

    rsx! {
        div {
            class: "bg-gradient-to-b from-[var(--web\\/graph\\/bar,#fcb300)] to-[var(--web\\/graph\\/bar2,#1a1a1a)] rounded-t-[10px] w-20",
            style: format!("height: {}px", bar_height),
        }
    }
}

fn aggregate_by_week(
    transactions: &[&PointTransactionResponse],
    year: i32,
    month: u32,
    days_in_month: u32,
) -> Vec<ChartBar> {
    let mut ranges = Vec::new();
    let mut start = 1u32;
    while start <= days_in_month {
        let end = (start + 6).min(days_in_month);
        ranges.push((start, end));
        start = end + 1;
    }

    let mut result = Vec::new();
    for (start, end) in &ranges {
        let mut total = 0i64;
        for tx in transactions {
            let ts = chrono::DateTime::from_timestamp(tx.created_at / 1000, 0).unwrap_or_default();
            let day = ts.date_naive().day();
            let tx_month = ts.date_naive().month();
            let tx_year = ts.date_naive().year();
            if tx_year == year && tx_month == month && day >= *start && day <= *end {
                total += tx.amount.abs();
            }
        }
        let label = format!("{}-{}", start, end);
        result.push(ChartBar {
            label,
            points: total,
        });
    }
    result
}

fn aggregate_by_day(
    transactions: &[&PointTransactionResponse],
    year: i32,
    month: u32,
    days_in_month: u32,
) -> Vec<ChartBar> {
    let mut daily = vec![0i64; days_in_month as usize];
    for tx in transactions {
        let ts = chrono::DateTime::from_timestamp(tx.created_at / 1000, 0).unwrap_or_default();
        let day = ts.date_naive().day();
        let tx_month = ts.date_naive().month();
        let tx_year = ts.date_naive().year();
        if tx_year == year && tx_month == month && day >= 1 && day <= days_in_month {
            daily[(day - 1) as usize] += tx.amount.abs();
        }
    }

    daily
        .into_iter()
        .enumerate()
        .map(|(i, pts)| ChartBar {
            label: format!("{}", i + 1),
            points: pts,
        })
        .collect()
}

fn aggregate_by_month(
    transactions: &[&PointTransactionResponse],
    month: &str,
) -> Vec<ChartBar> {
    let mut total = 0i64;
    for tx in transactions {
        total += tx.amount.abs();
    }
    vec![ChartBar {
        label: month.to_string(),
        points: total,
    }]
}
