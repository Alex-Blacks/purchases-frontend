<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../composables/useAuth'

const { token } = useAuth();

const newUserName = ref('');
const newUserPassword = ref('');
const newUserEmail = ref('');

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
</script>

<template>
    <div id="app">
        <AppTabs />
        <router-view />
    </div>
    <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Пользователи</h1>

        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Создать пользователя</h3>
            <input v-model="newUserName" placeholder="Имя" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="newUserPassword" placeholder="Пароль" type="password" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="newUserEmail" placeholder="Email" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleCreateUser">Создать</button>
            <p style="margin-top:8px;">{{ userResult }}</p>
        </div>

        <!-- Для отладки: показываем текущий токен -->
        <div v-if="token" style="background:#f0f0f0; padding:10px; border-radius:4px;">
            <strong>Токен:</strong> {{ token }}
        </div>
    </div>
</template>