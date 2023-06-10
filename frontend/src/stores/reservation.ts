import { defineStore } from 'pinia';
import { apiService } from '@/api';
import type { DayOverview, RosterEntry, Day } from '@/model';
import {RosterEntryType} from "@/model";
import router from "@/router"; // Assuming models.ts has the interface and type definitions

export const useStore = defineStore({
    id: 'mainStore',
    state: () => ({
        token: '',
        calendar: [] as DayOverview[],
        days: {} as Record<string, Day>,
    }),
    persist: true,
    actions: {
        async login(username: string, password: string) {
            this.token = await apiService.login(username, password);
        },
        async logout() {
            this.token = "";
            this.calendar = [];
            this.days = {};
            localStorage.removeItem('token');
            localStorage.removeItem('username');
            localStorage.removeItem('password');
            router.push('/login');
        },
        async getCalendar() {
            if (!this.token) throw new Error("Not authenticated");
            try {
                this.calendar = await apiService.getCalendar(this.token);
            } catch (ex) {
                if (ex === "unauthorized") {
                    this.logout();
                }
            }
        },
        async getDay(date: string) {
            if (!this.token) throw new Error("Not authenticated");
            this.days[date] = await apiService.getDay(date, this.token);
        },
        async updateDay(date: string, entryType: RosterEntryType, remarks?: string) {
            if (!this.token) throw new Error("Not authenticated");
            const day = this.days[date];
            day.entryType = entryType;
            day.remarks = remarks;
            await apiService.updateDay(date, this.token, day);
            await this.getDay(date);
        }
    },
});
