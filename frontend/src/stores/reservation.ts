import {defineStore} from 'pinia';
import {apiService} from '@/api';
import type {Day, DayOverview} from '@/model';
import {RosterEntryType} from "@/model";
import router from "@/router";
import type {AxiosError} from "axios";
import {isAxiosError} from "axios"; // Assuming models.ts has the interface and type definitions

export const useStore = defineStore({
    id: 'mainStore',
    state: () => ({
        token: '',
        calendar: [] as DayOverview[],
        days: {} as Record<string, Day>,
        dev: false
    }),
    persist: true,
    actions: {
        async login(username: string, password: string) {
            this.token = await apiService.login(username, password);
        },
        async checkLogin() {
            if (!this.token) return;
            try {
                const me = await apiService.me(this.token);
                console.log(me);
                if (me.id === "10000315") {
                    console.log("dev mode");
                    this.dev = true;
                }
            } catch (ex: any) {
                if (isAxiosError(ex)) {
                    const error = ex as AxiosError;
                    if (ex.response?.status === 401) {
                        await this.logout();
                    }
                }
            }
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
            } catch (ex: any | AxiosError) {
                if (isAxiosError(ex)) {
                    const error = ex as AxiosError;
                    // console.error(error);
                    if (error.response?.status === 401) {
                        if (localStorage.getItem('username') && localStorage.getItem('password')) {
                            await this.login(localStorage.getItem('username')!, localStorage.getItem('password')!);
                            await this.getCalendar();
                        } else {
                            await this.logout();
                        }
                    }
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
