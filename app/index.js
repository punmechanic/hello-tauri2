import { invoke } from "@tauri-apps/api/tauri";

async function fetchContainers() {
  return invoke("podman_api", { command: "fetch_containers" });
}

async function main() {
  const list = document.createElement("ol");
  document.body.appendChild(list);

  for (const container of await fetchContainers()) {
    const el = document.createElement("li");
    const text = `${container.id} - ${container.image_id}`;
    el.innerText = text;
    list.appendChild(el);
  }
}

main();
