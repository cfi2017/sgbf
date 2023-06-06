import axios from 'axios';
import type {AxiosInstance} from 'axios';
import type {Day, RosterEntry} from "@/model";

class ApiService {
    private instance: AxiosInstance;

    constructor() {
        this.instance = axios.create({
            baseURL: import.meta.env.VITE_API_BASE_URL || '/api/reservation',
        });
    }

    public async login(username: string, password: string): Promise<string> {
        const response = await this.instance.post('/login', { username, password });
        return response.data.token;
    }

    public async getCalendar(token: string): Promise<Day[]> {
        const response = await this.instance.get('/calendar', {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        return response.data;
    }

    public async getDay(date: string, token: string): Promise<RosterEntry[]> {
        const response = await this.instance.get(`/day?date=${date}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        return response.data;
    }
}

export const apiService = new ApiService();
