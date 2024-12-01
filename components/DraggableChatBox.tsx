import React, { useState, useRef, useEffect } from 'react';
import { motion, useDragControls } from 'framer-motion';
import axios from 'axios';

interface Message {
  id: string;
  content: string;
  sender: 'user' | 'ai';
}

export const DraggableChatBox: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const dragControls = useDragControls();

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSendMessage = async () => {
    if (!inputMessage.trim()) return;

    const userMessage: Message = {
      id: `user-${Date.now()}`,
      content: inputMessage,
      sender: 'user'
    };

    setMessages(prev => [...prev, userMessage]);
    setInputMessage('');

    try {
      const response = await axios.post('https://8080-jyasuu-jyasuugithubio-i7qhgpif7nd.ws-us117.gitpod.io/api/chat', { message: inputMessage });
      
      const aiMessage: Message = {
        id: `ai-${Date.now()}`,
        content: response.data.message,
        sender: 'ai'
      };

      setMessages(prev => [...prev, aiMessage]);
    } catch (error) {
      console.error('Error sending message:', error);
      setMessages(prev => [...prev, {
        id: `error-${Date.now()}`,
        content: 'Sorry, something went wrong.',
        sender: 'ai'
      }]);
    }
  };

  return (
    <motion.div 
      drag
      dragControls={dragControls}
      dragListener={false}
      className="fixed bottom-4 right-4 z-50 w-80 bg-white shadow-lg rounded-lg border"
    >
      {/* Chat Header */}
      <div 
        className="bg-blue-500 text-white p-3 rounded-t-lg flex justify-between items-center"
        onPointerDown={(e) => dragControls.start(e)}
      >
        <h3>AI Chat</h3>
        <button 
          onClick={() => setIsOpen(!isOpen)}
          className="focus:outline-none"
        >
          {isOpen ? '▼' : '▲'}
        </button>
      </div>

      {/* Chat Body */}
      {isOpen && (
        <div className="h-96 flex flex-col">
          <div className="flex-grow overflow-y-auto p-4 space-y-2">
            {messages.map((msg) => (
              <div 
                key={msg.id} 
                className={`p-2 rounded-lg max-w-[80%] ${
                  msg.sender === 'user' 
                    ? 'bg-blue-100 self-end ml-auto' 
                    : 'bg-gray-100 self-start mr-auto'
                }`}
              >
                {msg.content}
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>

          {/* Input Area */}
          <div className="p-4 border-t flex">
            <input 
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSendMessage()}
              placeholder="Type a message..."
              className="flex-grow border rounded-l-lg p-2"
            />
            <button 
              onClick={handleSendMessage}
              className="bg-blue-500 text-white px-4 rounded-r-lg"
            >
              Send
            </button>
          </div>
        </div>
      )}
    </motion.div>
  );
};