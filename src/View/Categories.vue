<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAuth } from "../composables/useAuth";
import AppTabs from "../components/AppTabs.vue";

const { token } = useAuth();
const categoryName = ref('');
const categoryIDForGet = ref('')
const categoryIDForDelete = ref('')

const createCategoryResult = ref('')
const getCategoryResult = ref('')
const deleteCategoryResult = ref('')
const listCategoriesResult = ref('')

async function handleCreateCategory() {
    if(!token.value){
        createCategoryResult.value = 'Сначала войдите в систему';
        return;
    }
    if (!categoryName.value) {
        createCategoryResult.value = 'Введите название категории';
        return;
    }
    try {
        const result = await invoke('create_category', {
            token: token.value,
            name: categoryName.value
        });
        createCategoryResult.value = `Категория создана: ${JSON.stringify(result)}`;
    } catch ( error ) {
        createCategoryResult.value = `Ошибка: ${error}`;
    }
}

async function handleGetCategory() {
    if(!token.value){
        getCategoryResult.value = 'Сначала войдите в систему';
        return;
    }
    const id = Number(categoryIDForGet.value)
    if(!categoryIDForGet.value || isNaN(id) || id <= 0){
        getCategoryResult.value = 'Введите корректный числовой ID категории';
        return;
    }
    try {
        const result = await invoke('get_category', {
            token: token.value,
            id: id
        });
        getCategoryResult.value = `Категория: ${JSON.stringify(result)}`;
    } catch( error ) {
        getCategoryResult.value = `Ошибка: ${error}`;
    }
}

async function handleDeleteCategory() {
    if(!token.value){
        deleteCategoryResult.value = 'Сначала войдите в систему';
        return;
    }
    const id = Number(categoryIDForDelete.value)
    if(!categoryIDForDelete.value || isNaN(id) || id <= 0){
        deleteCategoryResult.value = 'Введите корректный числовой ID категории';
        return;
    }
    try {
        const result = await invoke('delete_category', {
            token: token.value,
            id: id
        });
        deleteCategoryResult.value = `Категория: ${JSON.stringify(result)}`;
    } catch( error ) {
        deleteCategoryResult.value = `Ошибка: ${error}`;
    }
}

async function handleListCategories() {
    if(!token.value){
        listCategoriesResult.value = 'Сначала войдите в систему';
        return;
    }
    try {
        const result = await invoke('list_categories', {
            token: token.value
        });
        listCategoriesResult.value = `Список категорий: ${JSON.stringify(result)}`;
    } catch( error ) {
        listCategoriesResult.value = `Ошибка: ${error}`;
    }
}
</script>
<template>
    <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Категории</h1>

        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Создать категорию</h3>
            <input v-model="categoryName" placeholder="Название категории" style="display: block; margin-bottom: 8px; width: 100%;" />
            <button @click="handleCreateCategory">Создать</button>
            <p style="margin-top: 8px;">{{ createCategoryResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Посмотреть категорию</h3>
            <input v-model="categoryIDForGet" placeholder="ID категории" style="display: block; margin-bottom: 8px; width: 100%;" />
            <button @click="handleGetCategory">Посмотреть</button>
            <p style="margin-top: 8px;">{{ getCategoryResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить категорию</h3>
            <input v-model="categoryIDForDelete" placeholder="ID категории" style="display: block; margin-bottom: 8px; width: 100%;" />
            <button @click="handleDeleteCategory">Удалить</button>
            <p style="margin-top: 8px;">{{ deleteCategoryResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Список категорий</h3>
            <button @click="handleListCategories">Посмотреть</button>
            <p style="margin-top: 8px;">{{ listCategoriesResult }}</p>
        </div>
        <div v-if="token" style="background:#f0f0f0; padding:10px; border-radius:4px;">
            <strong>Токен:</strong> {{ token }}
        </div>
    </div>
</template>