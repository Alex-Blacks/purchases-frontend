<template>
  <div class="tabs">
    <router-link 
      to="/login" 
      class="tab" 
      active-class="tab-active"
    >
      Авториция
    </router-link>
    <router-link 
      to="/stores" 
      class="tab" 
      active-class="tab-active"
    >
      Магазины
    </router-link>
    <router-link
      to="/categories"
      class="tab"
      active-class="tab-active"
    >
      Категории
    </router-link>
    <router-link
      to="/products"
      class="tab"
      active-class="tab-active"
    >
      Продукты
    </router-link>
    <router-link 
      to="/orders" 
      class="tab" 
      active-class="tab-active"
    >
      Заказы
    </router-link>
    <router-link 
      to="/users" 
      class="tab" 
      active-class="tab-active"
    >
      Пользователи
    </router-link>
  </div>
</template>

<style scoped>
.tabs {
  display: flex;
  gap: 8px;
  border-bottom: 1px solid #ddd;
  padding-bottom: 8px;
  margin-bottom: 20px;
}
.tab {
  padding: 8px 16px;
  cursor: pointer;
  text-decoration: none;
  color: #333;
  border-radius: 4px 4px 0 0;
}
.tab-active {
  background: #42b883;
  color: white;
}
</style>