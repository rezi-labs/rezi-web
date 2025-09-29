document.body.addEventListener("htmx:afterSwap", function (evt) {
  if (evt.detail.target.id === "chat-messages") {
    evt.detail.target.scrollTop = evt.detail.target.scrollHeight;
  }

  if (evt.detail.target.id === "todo-list") {
    evt.detail.target.scrollTop = evt.detail.target.scrollHeight;
  }
});
