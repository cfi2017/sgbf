import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import {createI18n} from "vue-i18n";
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import {createVuetify} from "vuetify";
import piniaPluginPersistedState, {createPersistedState} from "pinia-plugin-persistedstate";

const app = createApp(App)

import en from './locales/en.json';
import de from './locales/de.json';

type MessageSchema = typeof en;
const i18n = createI18n<[MessageSchema], 'en', 'de'>({
    locale: 'en', // set locale
    fallbackLocale: 'en', // set fallback locale
    legacy: false,
    messages: {
        en: en,
        de: de,
        // add other languages here
    },
});

const vuetify = createVuetify();

const pinia = createPinia();
pinia.use(createPersistedState({
    auto: true
}));
app.use(pinia)
app.use(router)
app.use(vuetify)
app.use(i18n)

app.mount('#app')
