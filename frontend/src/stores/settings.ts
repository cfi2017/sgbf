import {defineStore} from 'pinia';
import {useI18n} from "vue-i18n";
import {ref} from "vue";

export const useSettingsStore = defineStore('settingsStore', () => {

    const locale = ref('en');
    const i18n = useI18n();
    const setLocale = (l: string) => {
        console.log('setLocale', l)
        locale.value = l;
        i18n.locale = l;
        i18n.locale = 'de'
    }

    const loadSettings = () => {
        if (locale.value) {
            i18n.locale = locale.value;
        }
    }

    return {
        locale,
        setLocale,
        loadSettings,
        persist: true,
    };
});
