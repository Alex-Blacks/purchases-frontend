import { invoke } from "@tauri-apps/api/core";
import { useAuth } from "./useAuth";

export function useApi() {
    const { token, logout } = useAuth();

    async function call<T>(command: string, args?: Record<string, any>): Promise<T> {
        try {
            const finalArgs = args || {};
            if (token.value) {
                finalArgs.token = token.value
            }
            const result = await invoke<T>(command, finalArgs);
            return result;
        } catch ( error:any ) {
            if (error?.status === 401 || error?.code === 401){
                logout()
                throw new Error("Сессия истекла, пожалуйста, войдите заново");
            }
            throw error;
        }
    }

    async function listOrders() {
        return call<any[]>('list_orders');  
    }

    async function createUser(name: string, password: string, email: string, role: string) {
        return call('create_user', { name, password, email, role});  
    }

    async function getUser(id: number) {
        return call('get_user', { id });  
    }

    async function deleteUser(id: number) {
        return call('delete_user', { id });  
    }

    async function updateUser(id: number, updateData: any) {
        return call('update_user', { id, updateData });  
    }

    async function listUsers() {
        return call<any[]>('list_users');  
    }

    return {
        listOrders,
        createUser,
        getUser,
        deleteUser,
        updateUser,
        listUsers,
    }
}