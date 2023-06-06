import { defineStore } from 'pinia';

export const useLocaleStore = defineStore({
    id: 'localeStore',
    state: () => ({
        locale: 'en',
    }),
    persist: true,
});