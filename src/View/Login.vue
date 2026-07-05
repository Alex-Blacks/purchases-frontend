<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const loginEmail = ref('');
const loginPassword = ref('');
const token = ref('');

const loginResult = ref('');

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
</script>

<template>
  <div id="app">
    <AppTabs />
    <router-view />
  </div>
  <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
    <h1>Авториция</h1>

    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Вход</h3>
      <input v-model="loginEmail" placeholder="Email" style="display:block; margin-bottom:8px; width:100%;" />
      <input v-model="loginPassword" placeholder="Пароль" type="password" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="handleLogin">Войти</button>
      <p style="margin-top:8px; white-space:pre-wrap;">{{ loginResult }}</p>
    </div>

    <!-- Для отладки: показываем текущий токен -->
    <div v-if="token" style="background:#f0f0f0; padding:10px; border-radius:4px;">
      <strong>Токен:</strong> {{ token }}
    </div>
  </div>
</template>