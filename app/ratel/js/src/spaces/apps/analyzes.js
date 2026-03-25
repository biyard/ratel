import * as d3 from "d3";
import * as XLSX from "xlsx";

function getContainer(containerId) {
  if (!containerId) return null;
  return document.getElementById(String(containerId));
}

function measureChartWidth(container, fallback = 640) {
  if (!container) return fallback;
  const parentWidth =
    container.parentElement?.getBoundingClientRect?.().width ||
    container.parentElement?.clientWidth ||
    0;
  const ownWidth =
    container.getBoundingClientRect?.().width || container.clientWidth || 0;
  const measured = Math.max(ownWidth, parentWidth);
  return measured > 0 ? measured : fallback;
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
    (entry) => entry.count > 0 || entry.percentage > 0
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

  const width = measureChartWidth(container, 0);
  if (!width) {
    requestAnimationFrame(() => renderBarChart(req));
    return true;
  }
  const isMobile = width < 480;

  const root = d3
    .select(container)
    .append("div")
    .style("display", "flex")
    .style("flex-direction", "column")
    .style("gap", isMobile ? "10px" : "12px")
    .style("width", "100%")
    .attr("role", "img")
    .attr("aria-label", "Survey response bar chart");

  const rows = root
    .selectAll("div.chart-row")
    .data(entries)
    .enter()
    .append("div")
    .attr("class", "chart-row")
    .style("display", "flex")
    .style("flex-direction", "column")
    .style("gap", isMobile ? "6px" : "0");

  if (isMobile) {
    const meta = rows
      .append("div")
      .style("display", "flex")
      .style("align-items", "center")
      .style("justify-content", "space-between")
      .style("gap", "12px");

    meta
      .append("div")
      .style("color", "var(--foreground-muted)")
      .style("font-size", "12px")
      .style("font-weight", "500")
      .style("white-space", "nowrap")
      .text((entry) => truncateLabel(entry.label, 24));

    meta
      .append("div")
      .style("color", "var(--foreground-muted)")
      .style("font-size", "12px")
      .style("font-weight", "400")
      .style("white-space", "nowrap")
      .text((entry) => `${entry.count} (${entry.percentage.toFixed(1)}%)`);

    const track = rows
      .append("div")
      .style("width", "100%")
      .style("height", "12px")
      .style("border-radius", "4px")
      .style("background", "var(--border-separator)")
      .style("overflow", "hidden");

    track
      .append("div")
      .style("height", "100%")
      .style("border-radius", "4px")
      .style("width", (entry) => `${Math.max(0, Math.min(100, entry.percentage))}%`)
      .style("background", (entry) => entry.color);
  } else {
    const meta = rows
      .append("div")
      .style("display", "grid")
      .style("grid-template-columns", "40px minmax(0, 1fr) 92px")
      .style("align-items", "center")
      .style("gap", "12px");

    meta
      .append("div")
      .style("color", "var(--foreground-muted)")
      .style("font-size", "13px")
      .style("font-weight", "500")
      .style("white-space", "nowrap")
      .text((entry) => truncateLabel(entry.label));

    const track = meta
      .append("div")
      .style("width", "100%")
      .style("height", "10px")
      .style("border-radius", "4px")
      .style("background", "var(--border-separator)")
      .style("overflow", "hidden");

    track
      .append("div")
      .style("height", "100%")
      .style("border-radius", "4px")
      .style("width", (entry) => `${Math.max(0, Math.min(100, entry.percentage))}%`)
      .style("background", (entry) => entry.color);

    meta
      .append("div")
      .style("color", "var(--foreground-muted)")
      .style("font-size", "13px")
      .style("font-weight", "400")
      .style("white-space", "nowrap")
      .style("text-align", "right")
      .text((entry) => `${entry.count} (${entry.percentage.toFixed(1)}%)`);
  }

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

  const measuredWidth = measureChartWidth(container, 0);
  if (!measuredWidth) {
    requestAnimationFrame(() => renderPieChart(req));
    return true;
  }

  const size = Math.min(190, Math.max(160, measuredWidth));
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

  const chart = svg
    .append("g")
    .attr("transform", `translate(${size / 2}, ${size / 2})`);

  const pie = d3
    .pie()
    .sort(null)
    .value((entry) => entry.count);

  const arc = d3.arc().innerRadius(innerRadius).outerRadius(radius);
  const labelArc = d3
    .arc()
    .innerRadius(radius * 0.5)
    .outerRadius(radius * 0.5);
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
