document.body.addEventListener('htmx:afterSwap', function(evt) {
            if(evt.detail.target.id === 'chat-messages') {
              evt.detail.target.scrollTop = evt.detail.target.scrollHeight;
            }
});