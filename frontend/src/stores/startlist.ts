import {defineStore} from 'pinia';
import type {Start} from '@/model';
import {ref} from "vue";

export const startListStore = defineStore('startListStore', () => {

    // start list cache
    const list = ref([] as Start[]);

    // forms to provide caching in case the page crashes
    const forms = ref({
        a: {} as Start,
        b: {} as Start,
    })
    const pax = ref([] as string[]);
    const planes = ref([] as string[]);
    const pilots = ref([] as string[]);

    const addStart = (start: Start) => {
        list.value.push({
            ...start,
            updated: new Date()
        });
    }

    // loads the start list structure for the current day
    const load = async () => {

    }

    // saves the start list structure for the current day
    const save = async () => {
        await load();
    }

    return {
        list, forms, pax, planes, pilots,
        addStart, load, save,
        persist: true,
    }
});

