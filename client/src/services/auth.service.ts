import axios from "axios";
import { toaster } from "@/components/ui/toaster"

interface ILogin {
  username: string;
  password: string;
}

interface IRegister {
  username: string;
  password: string;
}

const BASE_API = "http://127.0.0.1:8000"

export async function login(email: string, password: string) {
  
  const user: ILogin = {
    username: email,
    password: password
  }
  
  try {
    const response = await axios.post(`${BASE_API}/login`, user);
       
    
    if (response.status === 200) {
      localStorage.setItem('token', response.data.token);
      localStorage.setItem('user_id', response.data.user_id);
      console.log(response.data)
    }
  } catch (error) {
    console.error('Error:', error.response ? error.response.data : error.message);
  }
}

export async function register(username: string, password: string, confirmPassword: string) {
  if (password !== confirmPassword) {
    toaster.create({
      description: "Password and confirm password do not match",
      type: "error",
    })
  }
  
  const user: IRegister = {
    username: username,
    password: password,
  }

  try {
    const response = await axios.post(`${BASE_API}/register`, user);
    console.log(response.data)

    if (response.status === 201) {
      localStorage.setItem('token', response.data.token);
      localStorage.setItem('user_id', response.data.user_id);
      console.log(response.data)
    }
  } catch (error) {
    console.error('Error:', error.response ? error.response.data : error.message);
  }
}

export async function getMessages() {
  try {
    const response = await axios.get(`${BASE_API}/messages`);
    console.log(response.data.messages)
    return response.data.messages;
  } catch (error) {
    console.error('Error:', error.response ? error.response.data : error.message);
  }
}


