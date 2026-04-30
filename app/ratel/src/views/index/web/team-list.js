const el = document.getElementById("home-teams-dd-list");
if (!el) {
  dioxus.send(false);
  return;
}
const nearBottom = el.scrollTop + el.clientHeight >= el.scrollHeight - 40;
dioxus.send(nearBottom);
