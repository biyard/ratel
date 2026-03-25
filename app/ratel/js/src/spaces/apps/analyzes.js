import * as d3 from "d3";
import * as XLSX from "xlsx";

function getContainer(containerId) {
  if (!containerId) return null;
  return document.getElementById(String(containerId));
}

function normalizeEntries(entries = []) {
  return entries.map((entry) => ({
    label: String(entry.label || ""),
    count: Number(entry.count || 0),
    percentage: Number(entry.percentage || 0),
    color: String(entry.color || "#6366f1"),
  }));
}

function nonZeroEntries(entries = []) {
  return normalizeEntries(entries).filter(
    (entry) => entry.count > 0 || entry.percentage > 0,
  );
}

function truncateLabel(label, maxLength = 16) {
  if (label.length <= maxLength) return label;
  return `${label.slice(0, maxLength - 1)}…`;
}

function renderEmptyState(container, minHeight = 180) {
  container.innerHTML = "";
  const empty = d3
    .select(container)
    .append("div")
    .style("min-height", `${minHeight}px`)
    .style("display", "flex")
    .style("align-items", "center")
    .style("justify-content", "center")
    .style("border", "1px solid var(--border-separator)")
    .style("border-radius", "9999px")
    .style("color", "var(--text-secondary)")
    .style("font-size", "12px")
    .style("font-weight", "600");

  empty.text("No Data");
}

function renderBarChart(req = {}) {
  const container = getContainer(req.container_id);
  if (!container) return false;

  const entries = normalizeEntries(req.entries);
  if (entries.length === 0) {
    renderEmptyState(container, 220);
    return true;
  }

  container.innerHTML = "";

  const width = Math.max(container.clientWidth || 640, 320);
  const rowHeight = 28;
  const topPadding = 0;
  const height = topPadding + entries.length * rowHeight;
  const labelWidth = 32;
  const gap = 10;
  const valueWidth = 84;
  const barWidth = Math.max(120, width - labelWidth - valueWidth - gap * 2);

  const svg = d3
    .select(container)
    .append("svg")
    .attr("width", "100%")
    .attr("height", height)
    .attr("viewBox", `0 0 ${width} ${height}`)
    .attr("role", "img")
    .attr("aria-label", "Survey response bar chart");

  const x = d3.scaleLinear().domain([0, 100]).range([0, barWidth]);

  const row = svg
    .selectAll("g.chart-row")
    .data(entries)
    .enter()
    .append("g")
    .attr("class", "chart-row")
    .attr("transform", (_, index) => `translate(0, ${topPadding + index * rowHeight})`);

  row
    .append("text")
    .attr("x", 0)
    .attr("y", 16)
    .attr("fill", "var(--foreground-muted)")
    .attr("font-size", 13)
    .attr("font-weight", 500)
    .text((entry) => truncateLabel(entry.label));

  row
    .append("text")
    .attr("x", width)
    .attr("y", 16)
    .attr("text-anchor", "end")
    .attr("fill", "var(--foreground-muted)")
    .attr("font-size", 13)
    .attr("font-weight", 400)
    .text((entry) => `${entry.count} (${entry.percentage.toFixed(1)}%)`);

  row
    .append("rect")
    .attr("x", labelWidth + gap)
    .attr("y", 4)
    .attr("width", barWidth)
    .attr("height", 10)
    .attr("rx", 4)
    .attr("fill", "var(--border-separator)");

  row
    .append("rect")
    .attr("x", labelWidth + gap)
    .attr("y", 4)
    .attr("width", (entry) => x(Math.max(0, Math.min(100, entry.percentage))))
    .attr("height", 10)
    .attr("rx", 4)
    .attr("fill", (entry) => entry.color);

  return true;
}

function renderPieChart(req = {}) {
  const container = getContainer(req.container_id);
  if (!container) return false;

  const entries = nonZeroEntries(req.entries);
  if (entries.length === 0) {
    container.innerHTML = "";
    return true;
  }

  container.innerHTML = "";

  const size = 190;
  const radius = size / 2 - 8;
  const innerRadius = 0;

  const svg = d3
    .select(container)
    .append("svg")
    .attr("width", size)
    .attr("height", size)
    .attr("viewBox", `0 0 ${size} ${size}`)
    .attr("role", "img")
    .attr("aria-label", "Survey response pie chart")
    .style("display", "block")
    .style("margin", "0 auto");

  const chart = svg.append("g").attr("transform", `translate(${size / 2}, ${size / 2})`);

  const pie = d3
    .pie()
    .sort(null)
    .value((entry) => entry.count);

  const arc = d3.arc().innerRadius(innerRadius).outerRadius(radius);
  const labelArc = d3.arc().innerRadius(radius * 0.5).outerRadius(radius * 0.5);
  const pieData = pie(entries);

  chart
    .selectAll("path.slice")
    .data(pieData)
    .enter()
    .append("path")
    .attr("class", "slice")
    .attr("d", arc)
    .attr("fill", (entry) => entry.data.color)
    .attr("stroke", "var(--border-separator)")
    .attr("stroke-width", 1);

  const labels = chart
    .selectAll("text.slice-label")
    .data(pieData)
    .enter()
    .append("text")
    .attr("class", "slice-label")
    .attr("transform", (entry) => `translate(${labelArc.centroid(entry)})`)
    .attr("text-anchor", "middle")
    .attr("dominant-baseline", "central")
    .attr("fill", "#ffffff")
    .style("font-size", "12px")
    .style("font-weight", 500);

  labels
    .append("tspan")
    .attr("x", 0)
    .attr("dy", "-0.2em")
    .text((entry) => `${entry.data.label}: ${entry.data.count}`);

  labels
    .append("tspan")
    .attr("x", 0)
    .attr("dy", "1.2em")
    .text((entry) => `${entry.data.percentage.toFixed(0)}%`);

  return true;
}

async function downloadExcel(req = {}) {
  const fileName = String(req.file_name || "poll-analysis.xlsx");
  const sheetName = String(req.sheet_name || "Responses");
  const rows =
    Array.isArray(req.rows) && req.rows.length > 0
      ? req.rows
      : [["ID", "조사구분", "유형", "질문지"]];
  const merges = Array.isArray(req.merges) ? req.merges : [];

  const worksheet = XLSX.utils.aoa_to_sheet(rows);
  worksheet["!merges"] = merges.map((merge) => ({
    s: { r: Number(merge.start_row || 0), c: Number(merge.start_col || 0) },
    e: { r: Number(merge.end_row || 0), c: Number(merge.end_col || 0) },
  }));
  worksheet["!cols"] = rows[0].map((_, index) => {
    if (index === 0) return { wch: 24 };
    if (index === 1) return { wch: 18 };
    if (index === 2) return { wch: 10 };
    if (index === 3) return { wch: 48 };
    return { wch: 32 };
  });

  const workbook = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(workbook, worksheet, sheetName);
  XLSX.writeFileXLSX(workbook, fileName, { compression: true });

  return true;
}

const analyzes = {
  downloadExcel,
  renderBarChart,
  renderPieChart,
};

export default analyzes;
