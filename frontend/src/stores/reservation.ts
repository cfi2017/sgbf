import { defineStore } from 'pinia';
import { apiService } from '@/api';
import type { Day, RosterEntry } from '@/model'; // Assuming models.ts has the interface and type definitions

export const useStore = defineStore({
    id: 'mainStore',
    state: () => ({
        token: '',
        calendar: [] as Day[],
        days: {} as Record<string, RosterEntry[]>,
    }),
    persist: true,
    actions: {
        async login(username: string, password: string) {
            this.token = await apiService.login(username, password);
        },
        async getCalendar() {
            if (!this.token) throw new Error("Not authenticated");
            this.calendar = await apiService.getCalendar(this.token);
        },
        async getDay(date: string) {
            if (!this.token) throw new Error("Not authenticated");
            this.days[date] = await apiService.getDay(date, this.token);
        },
    },
});
