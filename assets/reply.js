let activeReplyId = null;

function setActiveReply(messageId, content, previewContent) {
    // Clear any previous active reply
    clearReply();
    
    // Set the new active reply
    activeReplyId = messageId;
    document.getElementById('reply-to-id').value = messageId;
    
    // Show reply context
    document.getElementById('reply-context').classList.remove('hidden');
    document.getElementById('reply-preview').textContent = previewContent.length > 50 ? previewContent.substring(0, 50) + '...' : previewContent;
    
    // Highlight the reply button and add visual feedback
    const replyBtn = document.getElementById('reply-btn-' + messageId);
    if (replyBtn) {
        // Remove active class from all reply buttons
        document.querySelectorAll('.reply-btn').forEach(btn => {
            btn.classList.remove('btn-primary', 'btn-active');
            btn.classList.add('btn-ghost');
        });
        
        // Add active class to current reply button
        replyBtn.classList.remove('btn-ghost');
        replyBtn.classList.add('btn-primary', 'btn-active');
        
        // Add a visual indicator to the message being replied to
        const messageElement = replyBtn.closest('.chat');
        if (messageElement) {
            messageElement.classList.add('ring-2', 'ring-primary', 'ring-opacity-50');
        }
    }
    
    // Focus on the input
    document.getElementById('reply-input').focus();
}

function clearReply() {
    // Clear reply context
    document.getElementById('reply-context').classList.add('hidden');
    document.getElementById('reply-to-id').value = '';
    document.getElementById('reply-preview').textContent = '';
    
    // Reset all reply buttons to normal state
    document.querySelectorAll('.reply-btn').forEach(btn => {
        btn.classList.remove('btn-primary', 'btn-active');
        btn.classList.add('btn-ghost');
    });
    
    // Remove visual indicators from all messages
    document.querySelectorAll('.chat').forEach(msg => {
        msg.classList.remove('ring-2', 'ring-primary', 'ring-opacity-50');
    });
    
    activeReplyId = null;
}