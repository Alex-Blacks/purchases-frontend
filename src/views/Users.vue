<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../composables/useAuth'

const { token } = useAuth();

const userNameForCreate = ref('');
const userPasswordForCreate = ref('');
const userEmailForCreate = ref('');

const userIDForGet = ref('');
const userIDForDelete = ref('');
const userIDForUpdate = ref('');

const userNameForUpdate = ref('');
const userPasswordForUpdate = ref('');
const userEmailForUpdate = ref('');
const userRoleForUpdate = ref('');
const userStatusForUpdate = ref('');

const createUserResult = ref('');
const getUserResult = ref('');
const deleteUserResult = ref('');
const updateUserResult = ref('');
const listUsersResult = ref('');

async function handleCreateUser() {
  if (!token.value) {
    createUserResult.value = 'Сначала войдите в систему';
    return;
  }
  if (!userNameForCreate.value || !userPasswordForCreate.value || !userEmailForCreate.value) {
    createUserResult.value = 'Укажите все данные';
    return;
  }
  try {
    const result = await invoke('create_user', {
      name: userNameForCreate.value,
      password: userPasswordForCreate.value,
      email: userEmailForCreate.value,
      role: 'user'
    });
    createUserResult.value = `Пользователь создан: ${JSON.stringify(result)}`;
  } catch (error) {
    createUserResult.value = `Ошибка: ${error}`;
  }
}

async function handleGetUser() {
  if (!token.value) {
    getUserResult.value = 'Сначала войдите в систему';
    return;
  }
  const id = Number(userIDForGet.value);
  if (!userIDForGet.value || isNaN(id) || id <= 0) {
    getUserResult.value = 'Введите корректный ID';
    return;
  }
  try {
    const result = await invoke('get_user', {
      token: token.value,
      id: id
    });
    getUserResult.value = `Пользователь: ${JSON.stringify(result)}`;
  } catch (error) {
    getUserResult.value = `Ошибка: ${error}`;
  }
}

async function handleDeleteUser() {
  if (!token.value) {
    deleteUserResult.value = 'Сначала войдите в систему';
    return;
  }
  const id = Number(userIDForDelete.value);
  if (!userIDForDelete.value || isNaN(id) || id <= 0) {
    deleteUserResult.value = 'Введите корректный ID';
    return;
  }
  try {
    await invoke('delete_user', {
      token: token.value,
      id: id
    });
    deleteUserResult.value = `Пользователь удалён`;
  } catch (error) {
    deleteUserResult.value = `Ошибка: ${error}`;
  }
}

async function handleUpdateUser() {
  if (!token.value) {
    updateUserResult.value = 'Сначала войдите в систему';
    return;
  }
  const id = Number(userIDForUpdate.value);
  if (!userIDForUpdate.value || isNaN(id) || id <= 0) {
    updateUserResult.value = 'Введите корректный ID';
    return;
  }
  const updateData: Record<string, string> = {};
  if (userNameForUpdate.value.trim()) updateData.name = userNameForUpdate.value.trim();
  if (userPasswordForUpdate.value.trim()) updateData.password = userPasswordForUpdate.value.trim();
  if (userEmailForUpdate.value.trim()) updateData.email = userEmailForUpdate.value.trim();
  if (userRoleForUpdate.value.trim()) updateData.role = userRoleForUpdate.value.trim();
  if (userStatusForUpdate.value.trim()) updateData.status = userStatusForUpdate.value.trim();

  if (Object.keys(updateData).length === 0) {
    updateUserResult.value = 'Укажите данные для обновления';
    return; 
  }
  try {
    const result = await invoke('update_user', {
      token: token.value,
      id: id,
      update: updateData
    });
    updateUserResult.value = `Пользователь обновлен: ${JSON.stringify(result)}`;
  } catch (error) {
    updateUserResult.value = `Ошибка: ${error}`;
  }
}

async function handleListeUsers() {
  if (!token.value) {
    listUsersResult.value = 'Сначала войдите в систему';
    return;
  }
  try {
    const result = await invoke('list_users', {
      token: token.value
    });
    listUsersResult.value = `Список пользователей: ${JSON.stringify(result)}`;
  } catch (error) {
    listUsersResult.value = `Ошибка: ${error}`;
  }
}
</script>

<template>
  <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
    <h1>Пользователи</h1>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Создать пользователя</h3>
      <input v-model="userNameForCreate" placeholder="Имя пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userPasswordForCreate" placeholder="Пароль пользователя" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userEmailForCreate" placeholder="Email пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleCreateUser">Создать</button>
      <p style="margin-top:8px;">{{ createUserResult }}</p>
    </div>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Посмотреть пользователя</h3>
      <input v-model="userIDForGet" placeholder="ID пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleGetUser">Посмотреть</button>
      <p style="margin-top:8px;">{{ getUserResult }}</p>
    </div>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Удалить пользователя</h3>
      <input v-model="userIDForDelete" placeholder="ID пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleDeleteUser">Удалить</button>
      <p style="margin-top:8px;">{{ deleteUserResult }}</p>
    </div>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Обновить пользователя</h3>
      <input v-model="userIDForUpdate" placeholder="ID пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userNameForUpdate" placeholder="Имя пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userPasswordForUpdate" placeholder="Пароль пользователя" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userEmailForUpdate" placeholder="Email пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userRoleForUpdate" placeholder="Роль пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="userStatusForUpdate" placeholder="Статус пользователя" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleUpdateUser">Обновить</button>
      <p style="margin-top:8px;">{{ updateUserResult }}</p>
    </div>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Список пользователей</h3>
      <button @click="handleListeUsers">Список</button>
      <p style="margin-top:8px;">{{ listUsersResult }}</p>
    </div>
  </div>
</template>