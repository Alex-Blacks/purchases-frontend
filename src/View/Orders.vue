<script setup lang="ts">
    import { ref } from "vue";
    import { invoke } from '@tauri-apps/api/core'
    import { useAuth } from '../composables/useAuth'

    const { token } = useAuth();

    const storeID = ref('');
    const orderIDForGet = ref('');
    const orderIDForDelete = ref('');

    const orderIDForItem = ref('');

    const productIDForAdd = ref('');
    const quantityForAdd = ref('');
    const productIDForUpdate = ref('');
    const quantityForUpdate = ref('');
    const productIDForDelete = ref('');

    const createOrderResult = ref('');
    const getOrderResult = ref('');
    const deleteOrderResult = ref('');
    const listOrdersResult = ref('');

    const addItemResult = ref('');
    const updateItemResult = ref('');
    const deleteItemResult = ref('');

    async function handlerCreateOrder() {
        if(!token.value) {
            createOrderResult.value = 'Сначала войдите в систему';
            return;
        }
        const id = Number(storeID.value);
        if(!storeID.value || isNaN(id) || id <=0) {
            createOrderResult.value = 'Введите корректный ID магазина';
            return;
        }
        try {
            const result = await invoke('create_order', {
                token: token.value,
                storeId: id
            });
            createOrderResult.value = `Заказ создан: ${JSON.stringify(result)}`;
        }catch ( error ) {
            createOrderResult.value = `Ошибка: ${error}`;
        }
    }

    async function handlerGetOrder() {
        if(!token.value) {
            getOrderResult.value = 'Сначала войдите в систему';
            return;
        }
        const id = Number(orderIDForGet.value);
        if(!orderIDForGet.value || isNaN(id) || id <=0) {
            getOrderResult.value = 'Введите корректный ID заказа';
            return;
        }
        try {
            const result = await invoke('get_order', {
                token: token.value,
                orderId: id
            });
            getOrderResult.value = `Заказ: ${JSON.stringify(result)}`;
        }catch ( error ) {
            getOrderResult.value = `Ошибка: ${error}`;
        }
    }

    async function handlerDeleteOrder() {
        if(!token.value) {
            deleteOrderResult.value = 'Сначала войдите в систему';
            return;
        }
        const id = Number(orderIDForDelete.value);
        if(!orderIDForDelete.value || isNaN(id) || id <=0) {
            deleteOrderResult.value = 'Введите корректный ID заказа';
            return;
        }
        try {
            await invoke('delete_order', {
                token: token.value,
                orderId: id
            });
            deleteOrderResult.value = `Заказ удалён`;
        }catch ( error ) {
            deleteOrderResult.value = `Ошибка: ${error}`;
        }
    }

    async function handlerListOrders() {
        if(!token.value) {
            listOrdersResult.value = 'Сначала войдите в систему';
            return;
        }
        try {
            const result = await invoke('list_orders', {
                token: token.value
            });
            listOrdersResult.value = `Список заказов: ${JSON.stringify(result)}`;
        }catch ( error ) {
            listOrdersResult.value = `Ошибка: ${error}`;
        }
    }

     async function handlerAddItem() {
        if(!token.value) {
            addItemResult.value = 'Сначала войдите в систему';
            return;
        }
        const orderID = Number(orderIDForItem.value);
        if(!orderIDForItem.value || isNaN(orderID) || orderID <=0) {
            addItemResult.value = 'Введите корректный ID заказа';
            return;
        }
        const productID = Number(productIDForAdd.value);
        if(!productIDForAdd.value || isNaN(productID) || productID <=0) {
            addItemResult.value = 'Введите корректный ID продукта';
            return;
        }
        const quantity = Number(quantityForAdd.value);
        if(!quantityForAdd.value || isNaN(quantity) || quantity <=0) {
            addItemResult.value = 'Введите корректное количество';
            return;
        }
        try {
            const result = await invoke('add_item', {
                token: token.value,
                orderId: orderID,
                productId: productID,
                quantity: quantity
            });
            addItemResult.value = `Товар добавлен: ${JSON.stringify(result)}`;
        }catch ( error ) {
            addItemResult.value = `Ошибка: ${error}`;
        }
    }

    async function handlerUpdateItem() {
        if(!token.value) {
            updateItemResult.value = 'Сначала войдите в систему';
            return;
        }
        const orderID = Number(orderIDForItem.value);
        if(!orderIDForItem.value || isNaN(orderID) || orderID <=0) {
            updateItemResult.value = 'Введите корректный ID заказа';
            return;
        }
        const productID = Number(productIDForUpdate.value);
        if(!productIDForUpdate.value || isNaN(productID) || productID <=0) {
            updateItemResult.value = 'Введите корректный ID продукта';
            return;
        }
        const quantity = Number(quantityForUpdate.value);
        if(!quantityForUpdate.value || isNaN(quantity) || quantity <=0) {
            updateItemResult.value = 'Введите корректное количество';
            return;
        }
        try {
            const result = await invoke('update_item', {
                token: token.value,
                orderId: orderID,
                productId: productID,
                quantity: quantity
            });
            updateItemResult.value = `Товар изменён: ${JSON.stringify(result)}`;
        }catch ( error ) {
            updateItemResult.value = `Ошибка: ${error}`;
        }
    }

    async function handlerDeleteItem() {
        if(!token.value) {
            deleteItemResult.value = 'Сначала войдите в систему';
            return;
        }
        const orderID = Number(orderIDForItem.value);
        if(!orderIDForItem.value || isNaN(orderID) || orderID <=0) {
            deleteItemResult.value = 'Введите корректный ID заказа';
            return;
        }
        const productID = Number(productIDForDelete.value);
        if(!productIDForDelete.value || isNaN(productID) || productID <=0) {
            deleteItemResult.value = 'Введите корректный ID продукта';
            return;
        }
        try {
            const result = await invoke('delete_item', {
                token: token.value,
                orderId: orderID,
                productId: productID
            });
            deleteItemResult.value = `Товар удалён: ${JSON.stringify(result)}`;
        }catch ( error ) {
            deleteItemResult.value = `Ошибка: ${error}`;
        }
    }
</script>
<template>
    <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Заказы</h1>

        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Создать заказ</h3>
            <input v-model="storeID" placeholder="ID магазина" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerCreateOrder">Создать</button>
            <p style="margin-top:8px;">{{ createOrderResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Посмотреть заказ</h3>
            <input v-model="orderIDForGet" placeholder="ID заказа" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerGetOrder">Посмотреть</button>
            <p style="margin-top:8px;">{{ getOrderResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить заказ</h3>
            <input v-model="orderIDForDelete" placeholder="ID заказа" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerDeleteOrder">Удалить</button>
            <p style="margin-top:8px;">{{ deleteOrderResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Список заказов</h3>
            <button @click="handlerListOrders">Просмотреть</button>
            <p style="margin-top:8px;">{{ listOrdersResult }}</p>
        </div>
    </div>
        <div style="padding: 20px; font-family: sans-serif; max-width: 600px; margin: 0 auto;">
        <h1>Элементы заказа</h1>
        <input v-model="orderIDForItem" placeholder="ID заказа" style="display:block; margin-bottom:8px; width:100%;" />
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Добавить товар</h3>
            <input v-model="productIDForAdd" placeholder="ID продукта" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="quantityForAdd" placeholder="Количество" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerAddItem">Добавить</button>
            <p style="margin-top:8px;">{{ addItemResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Изменить товар</h3>
            <input v-model="productIDForUpdate" placeholder="ID продукта" style="display:block; margin-bottom:8px; width:100%;" />
            <input v-model="quantityForUpdate" placeholder="Количество" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerUpdateItem">Изменить</button>
            <p style="margin-top:8px;">{{ updateItemResult }}</p>
        </div>
        <div style="border: 1px solid #ccc; padding: 45px; margin-bottom: 20px; border-radius: 8px;">
            <h3>Удалить товар</h3>
            <input v-model="productIDForDelete" placeholder="ID продукта" style="display:block; margin-bottom:8px; width:100%;" />
            <button @click="handlerDeleteItem">Удалить</button>
            <p style="margin-top:8px;">{{ deleteItemResult }}</p>
        </div>
    </div>
</template>