document.body.addEventListener('htmx:afterSwap', function(evt) {
  console.log( evt.detail.target.id, 'id');
            if(evt.detail.target.id === 'chat-messages') {
              console.log('Scrolling to bottom');
              evt.detail.target.scrollTop = evt.detail.target.scrollHeight;
            }
            
            if(evt.detail.target.id === 'todo-list') {
              console.log('Scrolling to bottom');
              evt.detail.target.scrollTop = evt.detail.target.scrollHeight;
            }
});