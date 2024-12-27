"use client"
import React, { useState } from 'react';
import {Button, Input, Tabs} from "@chakra-ui/react"
import styles from "./styles.module.scss";
import {Card} from "@chakra-ui/react"
import {Text} from "@chakra-ui/react"
import {login, register} from "@/services/auth.service";
import {useRouter} from "next/navigation";

export default function Home() {
  const router = useRouter();
  const [loginData, setFormLoginData] = useState({
    username: '',
    password: ''
  });

  const [registerData, setFormRegisterData] = useState({
    username: '',
    password: '',
    confirmPassword: ''
  });

  const handleRegisterChange = (e) => {
    const { name, value } = e.target;
    setFormRegisterData((prevData) => ({
      ...prevData,
      [name]: value
    }));
  };
  
  const handleLoginChange = (e) => {
    const { name, value } = e.target;
    setFormLoginData((prevData) => ({
      ...prevData,
      [name]: value
    }));
  };

  const handleRegisterSubmit = (e) => {
    e.preventDefault();
    console.log(registerData);
    register(registerData.username, registerData.password, registerData.confirmPassword);
    router.push("/chat");
  };
  
  const handleLoginSubmit = (e) => {
    e.preventDefault();
    console.log(loginData);
    login(loginData.username, loginData.password);
    router.push("/chat");
    
  };
  
  
  return (
    <div className={styles.container}>
      <div className={styles.inputs_block_container}>
        <Tabs.Root defaultValue="login" variant="plain" className={styles.tabs_contaier} size={"lg"}>
          <Tabs.List bg="bg.muted" rounded="l3" p="1">
            <Tabs.Trigger value="login" className={styles.tabs_trigger}>
              Login
            </Tabs.Trigger>
            <Tabs.Trigger value="register" className={styles.tabs_trigger}>
              Register
            </Tabs.Trigger>
            <Tabs.Indicator rounded="l2"/>
          </Tabs.List>
          <Tabs.Content value="login">
            <Card.Root width="320px" className={styles.card}>
              <Card.Body gap="2">
                <Card.Title mt="2">
                  <Text textStyle={"4xl"}>
                    Login
                  </Text>
                </Card.Title>
                <Card.Description className={styles.card_description}>
                  <div>
                    <Text textStyle={"lg"}>Enter your username</Text>
                    <Input
                      name="username"
                      value={loginData.username}
                      onChange={handleLoginChange}  
                      placeholder="Username"
                    />
                  </div>
                  <div>
                    <Text textStyle={"lg"}>Enter your password</Text>
                    <Input
                      name="password"
                      value={loginData.password}
                      onChange={handleLoginChange}
                      placeholder="Password"
                      type="password"
                    />
                  </div>
                </Card.Description>
              </Card.Body>
              <Card.Footer justifyContent="center">
                <Button size={"lg"} variant="subtle"  width={"40"} onClick={handleLoginSubmit}>Login</Button>
              </Card.Footer>
            </Card.Root>
          </Tabs.Content>
          <Tabs.Content value="register">
            <Card.Root width="320px" className={styles.card}>
              <Card.Body gap="2">
                <Card.Title mt="2">
                  <Text textStyle={"4xl"}>
                    Register
                  </Text>
                </Card.Title>
                <Card.Description className={styles.card_description}>
                  <div>
                    <Text textStyle={"lg"}>Enter your username</Text>
                    <Input
                      name="username"
                      value={registerData.username}
                      onChange={handleRegisterChange}
                      placeholder="Username"
                    />
                  </div>
                  <div>
                    <Text textStyle={"lg"}>Enter your password</Text>
                    <Input
                      name="password"
                      value={registerData.password}
                      onChange={handleRegisterChange}  
                      placeholder="Password"
                      type="password"
                    />
                  </div>
                  <div>
                    <Text textStyle={"lg"}>Enter your confirm password</Text>
                    <Input
                      name="confirmPassword"
                      value={registerData.confirmPassword}
                      onChange={handleRegisterChange}
                      placeholder="Confirm password"
                      type="password"
                    />
                  </div>
                </Card.Description>
              </Card.Body>
              <Card.Footer justifyContent="center">
                <Button size={"lg"} variant="subtle" width={"40"} onClick={handleRegisterSubmit}>Register</Button>
              </Card.Footer>
            </Card.Root>
          </Tabs.Content>
        </Tabs.Root>
      </div>
    </div>
  );
}
