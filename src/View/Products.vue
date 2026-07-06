<script setup lang="ts">
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from '../composables/useAuth'

const { token } = useAuth();

const productTitle = ref('');
const productUnit = ref('');
const productCategoryID = ref('');

const productIDForGet = ref('');
const productIDForDelete = ref('');
const productIDForFinc = ref('');

const productAlias = ref('');
const productIDForAlias = ref('');
const aliasIDForGet = ref('');
const aliasIDForDelete = ref('');
const aliasForFind = ref('');


const createProductResult = ref('');
const getProductResult = ref('');
const deleteProductResult = ref('');
const listProductResult = ref('');

const createProductAliasResult = ref('');
const getProductAliasResult = ref('');
const deleteProductAliasResult = ref('');
const deleteAllProductAliasResult = ref('');
const listProductAliasResult = ref('');
const findProductByAliasResult = ref('');

async function handleCreateProduct() {
    if(!token.value) {
        createProductResult.value = 'Сначала войдите в систему';
        return;
    }
    if(!productTitle.value || !productUnit.value || !productCategoryID.value) {
        createProductResult.value = 'Укажите все данные';
        return;
    }
    const categoryID = Number(productCategoryID.value);
    if(!productCategoryID.value || isNaN(categoryID) || categoryID<=0){
        createProductResult.value = 'Введите корректный ID категории';
        return;
    }

    try {
        const result = await invoke('create_product', {
            token: token.value,
            title: productTitle.value,
            unit: productUnit.value,
            category_id: categoryID
        });
        createProductResult.value = `Продукт создан: ${JSON.stringify(result)}`;
    } catch ( error ) {
        createProductResult.value = `Ошибка: ${error}`;
    }
}

async function handleGetProduct() {
    if(!token.value) {
        getProductResult.value = 'Сначала войдите в систему';
        return;
    }
    const id = Number(productIDForGet.value);
    if(!productIDForGet.value || isNaN(id) || id<=0) {
        getProductResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    try {
        const result = await invoke('get_product', {
            token: token.value,
            id: id
        });
        getProductResult.value = `Продукт: ${JSON.stringify(result)}`;
    } catch ( error ) {
        getProductResult.value = `Ошибка: ${error}`;
    }
}

async function handleDeleteProduct() {
    if(!token.value) {
        deleteProductResult.value = 'Сначала войдите в систему';
        return;
    }
    const id = Number(productIDForDelete.value);
    if(!productIDForDelete.value || isNaN(id) || id<=0) {
        deleteProductResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    try {
        await invoke('delete_product', {
            token: token.value,
            id: id
        });
        deleteProductResult.value = `Продукт удалён`;
    } catch ( error ) {
        deleteProductResult.value = `Ошибка: ${error}`;
    }
}

async function handleListProducts() {
    if(!token.value) {
        listProductResult.value = 'Сначала войдите в систему';
        return;
    }
    try {
        const result = await invoke('list_products', {
            token: token.value
        });
        listProductResult.value = `Список продуктов: ${JSON.stringify(result)}`;
    } catch ( error ) {
        listProductResult.value = `Ошибка: ${error}`;
    }
}

async function handleFindProductByAlias() {
    if(!token.value) {
        findProductByAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForFinc.value);
    if(!productIDForFinc.value || isNaN(productID) || productID<=0) {
        findProductByAliasResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    if(!aliasForFind.value){
        findProductByAliasResult.value = 'Введите алиас'
    }
    try {
        const result = await invoke('find_product_by_alias', {
            token: token.value,
            product_id: productID,
            alias: aliasForFind.value
        });
        findProductByAliasResult.value = `Продукт: ${JSON.stringify(result)}`;
    } catch ( error ) {
        findProductByAliasResult.value = `Ошибка: ${error}`;
    }
}

// Product Alias

async function handleCreateProductAlias() {
    if(!token.value) {
        createProductAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForAlias.value)
    if(!productAlias.value || isNaN(productID) || productID<= 0) {
        createProductAliasResult.value = 'Введите все данные или проверьте их корректность';
        return;
    }
    try {
        const result = await invoke('create_product_alias', {
            token: token.value,
            product_id: productID,
            alias: productAlias.value
        });
        createProductAliasResult.value = `Алиас создан: ${JSON.stringify(result)}`;
    } catch ( error ) {
        createProductAliasResult.value = `Ошибка: ${error}`;
    }
}

async function handleGetProductAlias() {
    if(!token.value) {
        getProductAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForAlias.value);
    if(!productIDForGet.value || isNaN(productID) || productID<=0) {
        getProductAliasResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    const id = Number(aliasIDForGet.value);
    if(!aliasIDForGet.value || isNaN(id) || id<=0) {
        getProductAliasResult.value = 'Введите корректный числовой ID алиаса';
        return;
    }
    try {
        const result = await invoke('get_product_alias', {
            token: token.value,
            product_id: productID,
            id: id
        });
        getProductAliasResult.value = `Алиас: ${JSON.stringify(result)}`;
    } catch ( error ) {
        getProductAliasResult.value = `Ошибка: ${error}`;
    }
}

async function handleDeleteProductAlias() {
    if(!token.value) {
        deleteProductAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForAlias.value);
    if(!productIDForAlias.value || isNaN(productID) || productID<=0) {
        deleteProductAliasResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    const id = Number(aliasIDForDelete.value);
    if(!aliasIDForDelete.value || isNaN(id) || id<=0) {
        deleteProductAliasResult.value = 'Введите корректный числовой ID алиаса';
        return;
    }
    try {
        await invoke('delete_product_alias', {
            token: token.value,
            product_id: productID,
            id: id
        });
        deleteProductAliasResult.value = `Алиас удалён`;
    } catch ( error ) {
        deleteProductAliasResult.value = `Ошибка: ${error}`;
    }
}

async function handleDeleteAllProductAliases() {
    if(!token.value) {
        deleteAllProductAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForAlias.value);
    if(!productIDForAlias.value || isNaN(productID) || productID<=0) {
        deleteAllProductAliasResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    try {
        await invoke('delete_all_product_aliases', {
            token: token.value,
            product_id: productID
        });
        deleteAllProductAliasResult.value = `Все алиасы продукта удалены`;
    } catch ( error ) {
        deleteAllProductAliasResult.value = `Ошибка: ${error}`;
    }
}

async function handleListProductAliases() {
    if(!token.value) {
        listProductAliasResult.value = 'Сначала войдите в систему';
        return;
    }
    const productID = Number(productIDForAlias.value);
    if(!productIDForAlias.value || isNaN(productID) || productID<=0) {
        listProductAliasResult.value = 'Введите корректный числовой ID продукта';
        return;
    }
    try {
        const result = await invoke('list_product_aliases', {
            token: token.value,
            product_id: productID
        });
        listProductAliasResult.value = `Список продуктов: ${JSON.stringify(result)}`;
    } catch ( error ) {
        listProductAliasResult.value = `Ошибка: ${error}`;
    }
}
</script>
<template>
    <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Продукты</h1>

        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Создать продукт</h3>
            <input v-model="productTitle" placeholder="Название продукта" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="productUnit" placeholder="В чём измерятся" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="productCategoryID" placeholder="ID категории" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleCreateProduct">Создать</button>
            <p style="margin-top:8px;">{{ createProductResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Посмотреть продукт</h3>
            <input v-model="productIDForGet" placeholder="ID продукта" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleGetProduct">Посмотреть</button>
            <p style="margin-top:8px;">{{ getProductResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить продукт</h3>
            <input v-model="productIDForDelete" placeholder="ID магазина" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleDeleteProduct">Удалить</button>
            <p style="margin-top:8px;">{{ deleteProductResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Список продуктов</h3>
            <button @click="handleListProducts">Просмотреть</button>
            <p style="margin-top:8px;">{{ listProductResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Поиск продукта по алиасу</h3>
            <input v-model="aliasForFind" placeholder="Алиас" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleFindProductByAlias">Просмотреть</button>
            <p style="margin-top:8px;">{{ findProductByAliasResult }}</p>
        </div>
    </div>
        <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Алиасы продуктов</h1>
        <input v-model="productIDForAlias" placeholder="ID продукта" style="display:block; margin-bottom:8px; width:100%;" />
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Создать алиас</h3>
            <input v-model="productAlias" placeholder="Алиас" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleCreateProductAlias">Создать</button>
            <p style="margin-top:8px;">{{ createProductAliasResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Посмотреть алиас</h3>
            <input v-model="aliasIDForGet" placeholder="ID алиаса" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleGetProductAlias">Посмотреть</button>
            <p style="margin-top:8px;">{{ getProductAliasResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить алиас</h3>
            <input v-model="aliasIDForDelete" placeholder="ID алиаса" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handleDeleteProductAlias">Удалить</button>
            <p style="margin-top:8px;">{{ deleteProductAliasResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить все алиасы у продукта</h3>
            <button @click="handleDeleteAllProductAliases">Удалить</button>
            <p style="margin-top:8px;">{{ deleteAllProductAliasResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Список алиасов</h3>
            <button @click="handleListProductAliases">Просмотреть</button>
            <p style="margin-top:8px;">{{ listProductAliasResult }}</p>
        </div>

        <!-- Для отладки: показываем текущий токен -->
        <div v-if="token" style="background:#f0f0f0; padding:10px; border-radius:4px;">
            <strong>Токен:</strong> {{ token }}
        </div>
    </div>
</template>