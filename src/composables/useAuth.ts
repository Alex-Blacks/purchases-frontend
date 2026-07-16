import { ref, readonly } from "vue";
import { invoke } from "@tauri-apps/api/core";

const token = ref<string|null>('')
const isAuthenticated = ref(false);

async function login(email: string, password: string): Promise<void> {
    try {
        let result = await invoke<string>('login', { email, password });
        token.value = result;
        isAuthenticated.value = true;
        localStorage.setItem('token', token.value);
    } catch ( error ) {
        throw new Error(`Ошибка: ${error}`);
    }
}

async function register(name: string, email: string, password: string) {
    try {
        await invoke ('register', { name, email, password });
        login(email, password);
    } catch ( error ) {
        throw new Error(`Ошибка: ${error}`);
    }
}

async function logout() {
    token.value = '';
    isAuthenticated.value = false;
}

async function initAuth() {
    token.value = localStorage.getItem('token');
    isAuthenticated.value = !!token.value;
}

export function useAuth() {
    return {
        token: readonly(token),
        isAuthenticated: readonly(isAuthenticated),
        login,
        register,
        logout,
        initAuth        
    }
}