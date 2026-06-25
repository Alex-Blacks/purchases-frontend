<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const newUserName = ref('');
const newUserPassword = ref('');
const newUserEmail = ref('');

const loginEmail = ref("");
const loginPassword = ref("");
const token = ref("");

const storeName = ref('');
const createStoreResult = ref('');
const userResult = ref('');

async function handleCreateUser() {
  try {
    const result = await invoke('create_user', {
      name: newUserName.value,
      password: newUserPassword.value,
      email: newUserEmail.value,
      role: 'user'
    });
    userResult.value = `Пользователь создан: ${JSON.stringify(result)}`;
  } catch (error) {
    userResult.value = `Ошибка: ${error}`;
  }
}

async function handleLogin() {
  try {
    const result = await invoke('login', { 
      email: loginEmail.value, 
      password: loginPassword.value
    });
    token.value = t as string;
    userResult.value = `Успешный вход! Токен: ${token.value}`;
  } catch (error) {
    userResult.value = `Ошибка входа: ${error}`;
  }
}

async function handleCreateStore() {
  try {
    const result = await invoke('create_store', {
      token: token.value,
      name: storeName.value
    });
    createStoreResult.value = `Магазин создан: ${JSON.stringify(store)}`;
  } catch (error) {
    createStoreResult.value = `Ошибка: ${error}`;
  } 
}
</script>

<template>
  <div style="display:flex; flex-direction: column; gap: 20px; padding: 20px">
    <div>
      <h3>Создать пользователя</h3>
      <input v-model="newUserName" placeholder="Имя" />
      <input v-model="newUserPassword" placeholder="Пароль" type="password" />
      <input v-model="newUserEmail" placeholder="Email" />
      <button @click="handleCreateUser">Создать</button>
      <p>{{ userResult }}</p>
    </div>
    
    <div>
      <h3>Вход</h3>
      <input v-model="loginEmail" placeholder="Email" />
      <input v-model="loginPassword" placeholder="Пароль" type="password" />
      <button @click="handleLogin">Войти</button>
    </div>

    <div>
      <h3>Создать магазин</h3>
      <input v-model="storeName" placeholder="Название магазина" />
      <button @click="handleCreateStore">Создать магазин</button>
      <p>{{ createStoreResult }}</p>
    </div>

    <div v-if="token">
      <p><strong>Текущий токен:</strong> {{ token }}</p>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>