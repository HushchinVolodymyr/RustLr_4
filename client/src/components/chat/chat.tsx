"use client"
import React, {useEffect, useState} from 'react';
import {Button, Input, Stack, Text} from "@chakra-ui/react";
import styles from "./chat.module.scss";
import {getMessages} from "@/services/auth.service";
import {toaster} from "@/components/ui/toaster";

interface IMessage {
  id: number;
  text: string;
  user_id: number;
}

const Chat = () => {
  const [messages, setMessages] = useState<IMessage[]>([]);  
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [message, setMessage] = useState<string>(''); 

  const userId = localStorage.getItem('user_id');

  const addMessage = (newMessage: IMessage) => {
    if (messages.some((msg) => msg.id === newMessage.id)) {
      return;
    }
    console.log(newMessage);
    setMessages((prevMessages) => [...prevMessages, newMessage]);
  };
  
  const get_data = async () => {
    const messages = await getMessages();
    messages.map((msg: IMessage) => {
      const newMessage: IMessage = JSON.parse(msg);
      addMessage(newMessage);
    })
  }
  useEffect(() => {
    get_data();
    
    const socketInstance = new WebSocket(`ws://localhost:8080/`);

    socketInstance.onopen = () => {
      console.log('WebSocket connection opened');
      toaster.create({
        description: "WebSocket connection opened",
        type: "success",
      })
      
      if (userId) {
        socketInstance.send(userId);
      }
    };

    socketInstance.onmessage = (event) => {
      console.log('Message receved:', event.data);

      try {
        const newMessage: IMessage = JSON.parse(event.data);
        addMessage(newMessage);

      } catch (error) {
        console.error('Error on message parsing:', error);
      }
    };

    socketInstance.onclose = () => {
      console.log('WebSocket connection closed');
    };

    setSocket(socketInstance);

    return () => {
      socketInstance.close();
    };
  }, [userId]);

  const sendMessage = () => {
    if (socket && message.trim()) {
      const messageToSend = {
        text: message,
        user_id: userId || 'unknown',
      };
      socket.send(JSON.stringify(messageToSend));  
      setMessage('');  
    }
  };

  return (
    <div className={styles.chat_container}>
      <ul className={styles.messages_container}>
        {messages.map((msg) => (
          <div key={msg.id}>
            <Text
              textStyle={"lg"}
              className={userId === msg.user_id ? styles.my_message : styles.other_message}
            >
              {msg.text}
            </Text>
          </div>
        ))}
      </ul>
      
      <Stack direction={"row"} className={styles.inputs_container}>
        <Input
          type="text"
          id="message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="Введите сообщение"
        />
        <Button onClick={sendMessage}>Отправить сообщение</Button>
      </Stack>
    </div>
  );
};

export default Chat;
