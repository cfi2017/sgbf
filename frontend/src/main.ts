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
import OneSignalVuePlugin from "@onesignal/onesignal-vue3";

type MessageSchema = typeof en;

const vuetify = createVuetify();

const pinia = createPinia();
pinia.use(createPersistedState({
    auto: true
}));
const i18n = createI18n<[MessageSchema], 'en', 'de'>({
    locale: JSON.parse((localStorage.getItem('settingsStore') || '{}')).locale || 'en', // set locale
    fallbackLocale: 'en', // set fallback locale
    legacy: false,
    messages: {
        'en': en,
        'de': de,
        // add other languages here
    },
});



app.use(pinia)
app.use(router)
app.use(vuetify)
app.use(i18n)
app.use(OneSignalVuePlugin, {
    appId: "597019c4-d476-4efa-9832-34791456301c",
    safari_web_id: "web.onesignal.auto.52bd6d36-ef00-42e1-a687-b4f3eaae4ff3",
    notifyButton: {
        enable: true,
    },
});

app.mount('#app')
