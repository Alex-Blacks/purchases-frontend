<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../composables/useAuth'

const { token } = useAuth();

const storeIdForGet = ref('');
const storeIdForDelete = ref(''); 
const storeName = ref('');

const createStoreResult = ref('');
const getStoreResult = ref('');
const deleteStoreResult = ref('');
const listStoresResult = ref('');

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
  <div id="app">
    <AppTabs />
    <router-view />
  </div>
  <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
    <h1>Магазины</h1>

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