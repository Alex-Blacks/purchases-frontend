<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const newUserName = ref('');
const newUserPassword = ref('');
const newUserEmail = ref('');

const loginEmail = ref('');
const loginPassword = ref('');
const token = ref('');

const storeIdForGet = ref('');
const storeIdForDelete = ref(''); 
const storeName = ref('');

const createStoreResult = ref('');
const getStoreResult = ref('');
const deleteStoreResult = ref('');
const listStoresResult = ref('');

const loginResult = ref('');
const userResult = ref('');

async function handleCreateUser() {
  if (!newUserName.value || !newUserPassword.value || !newUserEmail.value) {
    userResult.value = 'Заполните все поля для создания пользователя';
    return;
  }
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
  if (!loginEmail.value || !loginPassword.value) {
    loginResult.value = 'Введите email и пароль';
    return;
  }
  try {
    const result = await invoke('login', { 
      email: loginEmail.value, 
      password: loginPassword.value
    });
    token.value = result as string;
    loginResult.value = `Успешный вход! Токен получен.`;
  } catch (error) {
    loginResult.value = `Ошибка входа: ${error}`;
  }
}

async function handleCreateStore() {
  if (!token.value) {
    createStoreResult.value = 'Сначала войдите в систему';
    return;
  }
  if (!storeName.value) {
    createStoreResult.value = 'Введите название магазина';
    return;
  }
  try {
    const result = await invoke('create_store', {
      token: token.value,
      name: storeName.value
    });
    createStoreResult.value = `Магазин создан: ${JSON.stringify(result)}`;
  } catch (error) {
    createStoreResult.value = `Ошибка: ${error}`;
  } 
}

async function handleGetStore() {
  if (!token.value) {
    getStoreResult.value = 'Сначала войдите в систему';
    return;
  }
  const id = Number(storeIdForGet.value);
  if (!storeIdForGet.value || isNaN(id) || id <= 0) {
    getStoreResult.value = 'Введите корректный числовой ID магазина';
    return;
  }
  try {
    const result = await invoke('get_store', {
      token: token.value,
      id: id
    });
    getStoreResult.value = `Магазин: ${JSON.stringify(result)}`;
  } catch (error) {
    getStoreResult.value = `Ошибка: ${error}`;
  }
}

async function handleDeleteStore() {
  if (!token.value) {
    deleteStoreResult.value = 'Сначала войдите в систему';
    return;
  }
    const id = Number(storeIdForDelete.value);
  if (!storeIdForGet.value || isNaN(id) || id <= 0) {
    deleteStoreResult.value = 'Введите корректный числовой ID магазина';
    return;
  }
  try {
    await invoke('delete_store', {
      token: token.value,
      id: id
    });
    deleteStoreResult.value = 'Магазин удалён';
  } catch (error) {
    deleteStoreResult.value = `Ошибка: ${error}`;
  }
}

async function handleListStores() {
  if (!token.value) {
    listStoresResult.value = 'Сначала войдите в систему';
    return;
  }
  try {
    const result = await invoke('list_stores', {
      token: token.value,
    });
    listStoresResult.value = `Список магазинов: ${JSON.stringify(result)}`;
  } catch (error) {
    listStoresResult.value = `Ошибка: ${error}`;
  }
}
</script>
<template>
  <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
    <h1>Заказы</h1>

    <!-- БЛОК ЛОГИНА -->
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Вход</h3>
      <input v-model="loginEmail" placeholder="Email" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="loginPassword" placeholder="Пароль" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleLogin">Войти</button>
      <p style="margin-top:8px; white-space:pre-wrap;">{{ loginResult }}</p>
    </div>

    <!-- БЛОК СОЗДАНИЯ ПОЛЬЗОВАТЕЛЯ -->
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Создать пользователя</h3>
      <input v-model="newUserName" placeholder="Имя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="newUserPassword" placeholder="Пароль" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="newUserEmail" placeholder="Email" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleCreateUser">Создать</button>
      <p style="margin-top:8px;">{{ userResult }}</p>
    </div>

    <!-- БЛОК МАГАЗИНА -->
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Создать магазин</h3>
      <input v-model="storeName" placeholder="Название магазина" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleCreateStore">Создать магазин</button>
      <p style="margin-top:8px;">{{ createStoreResult }}</p>
    </div>
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Посмотреть магазин</h3>
      <input v-model="storeIdForGet" placeholder="ID магазина" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleGetStore">Посмотреть магазин</button>
      <p style="margin-top:8px;">{{ getStoreResult }}</p>
    </div>
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Удалить магазин</h3>
      <input v-model="storeIdForDelete" placeholder="ID магазина" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleDeleteStore">Удалить магазин</button>
      <p style="margin-top:8px;">{{ deleteStoreResult }}</p>
    </div>
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Список магазинов</h3>
      <button @click="handleListStores">Список магазинов</button>
      <p style="margin-top:8px;">{{ listStoresResult }}</p>
    </div>

    <!-- Для отладки: показываем текущий токен -->
    <div v-if="token" style="background:#f0f0f0; padding:10px; border-radius:4px;">
      <strong>Токен:</strong> {{ token }}
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
  padding: 10px 10px;
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