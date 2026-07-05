<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../../composables/useAuth'

const { token } = useAuth();
const storeID = ref('');
const result = ref('');

async function getStore() {
    if (!token.value) {
        result.value = 'Сначала войдите в систему';
        return;
    }
    const id = Number(storeID.value);
    if (!storeID.value || isNaN(id) || id <= 0) {
        result.value = 'Введите корректный числовой ID магазина';
        return;
    }
    try {
        const res = await invoke('get_store', {
            token: token.value,
            id: id
        });
        result.value = `Магазин: ${JSON.stringify(res)}`;
    } catch (e) {
        result.value = `Ошибка: ${e}`;
    }
}
</script>

<template>
    <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
      <h3>Посмотреть магазин</h3>
      <input v-model="storeID" placeholder="ID магазина" style="display:block; margin-bottom:8px; width:100%;" />
      <button @click="getStore">Посмотреть магазин</button>
      <p style="margin-top:8px;">{{ result }}</p>
    </div>
</template>