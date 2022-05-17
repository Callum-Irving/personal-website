if (navigator.clipboard) {
  document.querySelectorAll("pre > code").forEach((codeBlock) => {
    let button = document.createElement("button");
    button.className = "copy-button";
    button.type = "button";
    button.innerText = "Copy";
    button.addEventListener("click", copyCode);
    let pre = codeBlock.parentNode;
    pre.appendChild(button);
  });
}

async function copyCode(event) {
  const btn = event.srcElement;
  const pre = btn.parentElement;
  let code = pre.querySelector("code");
  let text = code.innerText;
  await navigator.clipboard.writeText(text);
  btn.blur();
  btn.innerText = "Copied!";
  setTimeout(() => (btn.innerText = "Copy"), 2000);
}
