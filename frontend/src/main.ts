import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import {createI18n} from "vue-i18n";
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import {createVuetify} from "vuetify";
import piniaPluginPersistedState from "pinia-plugin-persistedstate";

const app = createApp(App)

const i18n = createI18n({
    locale: 'en', // set locale
    fallbackLocale: 'en', // set fallback locale
    legacy: false,
    messages: {
        en: {
            // English translations go here
        },
        fr: {
            // French translations go here
        },
        // add other languages here
    },
});

const vuetify = createVuetify();

const pinia = createPinia();
pinia.use(piniaPluginPersistedState);
app.use(pinia)
app.use(router)
app.use(vuetify)
app.use(i18n)

app.mount('#app')
