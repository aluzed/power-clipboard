const { listen } = window.__TAURI__.event;

const stackList = document.getElementById("stack-list");

function renderStack(items, currentIndex) {
  stackList.innerHTML = "";

  if (!items || items.length === 0) {
    const li = document.createElement("li");
    li.className = "empty-message";
    li.textContent = "No items";
    stackList.appendChild(li);
    return;
  }

  items.forEach((item) => {
    const li = document.createElement("li");
    li.className = "item";
    if (item.index === currentIndex) {
      li.classList.add("selected");
    }
    li.textContent = item.preview;
    stackList.appendChild(li);
  });
}

listen("stack-navigated", (event) => {
  const { items, current_index } = event.payload;
  renderStack(items, current_index);
});

// Initial empty state
renderStack([], null);
