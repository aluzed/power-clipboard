const { invoke } = window.__TAURI__.core;

const stackLimitInput = document.getElementById("stack-limit");
const saveBtn = document.getElementById("save-btn");
const statusEl = document.getElementById("status");

async function loadSettings() {
  try {
    const settings = await invoke("get_settings");
    stackLimitInput.value = settings.stack_limit;
  } catch (e) {
    console.error("Failed to load settings:", e);
  }
}

saveBtn.addEventListener("click", async () => {
  const limit = parseInt(stackLimitInput.value, 10);
  if (isNaN(limit) || limit < 1 || limit > 50) {
    statusEl.textContent = "Please enter a value between 1 and 50";
    statusEl.style.color = "#c44";
    return;
  }

  try {
    await invoke("set_stack_limit", { limit });
    statusEl.textContent = "Saved!";
    statusEl.style.color = "#4a4";
    setTimeout(() => {
      statusEl.textContent = "";
    }, 2000);
  } catch (e) {
    statusEl.textContent = "Error saving settings";
    statusEl.style.color = "#c44";
    console.error(e);
  }
});

loadSettings();
