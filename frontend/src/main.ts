import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import {createI18n} from "vue-i18n";
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import {createVuetify} from "vuetify";
import { VDataTableVirtual } from 'vuetify/labs/VDataTable'
import piniaPluginPersistedState, {createPersistedState} from "pinia-plugin-persistedstate";

const app = createApp(App)

import en from './locales/en.json';
import de from './locales/de.json';
import OneSignalVuePlugin from "@onesignal/onesignal-vue3";
import {BrowserTracing, init, Replay, vueRouterInstrumentation} from "@sentry/vue";

type MessageSchema = typeof en;

const vuetify = createVuetify({
    components: {
        VDataTableVirtual
    },
    theme: {
        defaultTheme: 'sgbf',
        themes: {
            sgbf: {
                dark: false,
                colors: {
                    primary: '#252F79'
                }
            }
        }
    }
});

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

if (location.host === "sgbf.swiss.dev") {
    init({
        app,
        dsn: "https://8fa5bfe05d9a470c976d060d4431f41a@sentry.service.cloud.swiss.dev/4",
        integrations: [
            new BrowserTracing({
                // Set `tracePropagationTargets` to control for which URLs distributed tracing should be enabled
                tracePropagationTargets: ["localhost", /^https:\/\/sgbf\.swiss\.dev\/api/],
                routingInstrumentation: vueRouterInstrumentation(router),
            }),
            new Replay(),
        ],

        // Set tracesSampleRate to 1.0 to capture 100%
        // of transactions for performance monitoring.
        // We recommend adjusting this value in production
        tracesSampleRate: 0.1,

        // Capture Replay for 10% of all sessions,
        // plus for 100% of sessions with an error
        replaysSessionSampleRate: 0.1,
        replaysOnErrorSampleRate: 1.0,
    });
}

app.use(pinia)
app.use(router)
app.use(vuetify)
app.use(i18n)

if (location.host === "sgbf.swiss.dev") {
    app.use(OneSignalVuePlugin, {
        appId: "597019c4-d476-4efa-9832-34791456301c",
        safari_web_id: "web.onesignal.auto.52bd6d36-ef00-42e1-a687-b4f3eaae4ff3",
        notifyButton: {
            enable: true,
        }
    });
}

app.mount('#app')
