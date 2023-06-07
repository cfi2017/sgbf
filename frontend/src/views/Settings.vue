<template>
  <v-container>
    <v-card>
      <v-card-title>
        {{ t('settings.title') }}
      </v-card-title>
      <v-card-text>
        <v-list>
          <v-list-subheader>{{ t('settings.notifications.title') }}</v-list-subheader>
          <v-list-item>
            <v-checkbox v-model="settings.notifications" :label="t('settings.notifications.activate')"></v-checkbox>
            <v-list density="compact">
              <v-list-item>
                <v-checkbox :label="t('settings.notifications.subOptions.flightInstructors')"></v-checkbox>
              </v-list-item>
              <v-list-item>
                <v-checkbox :label="t('settings.notifications.subOptions.towPilots')"></v-checkbox>
              </v-list-item>
              <v-list-item>
                <v-checkbox :label="t('settings.notifications.subOptions.manyParticipants')"></v-checkbox>
              </v-list-item>
            </v-list>
          </v-list-item>
          <v-list-item>
            <v-select v-model="localeStuff.locale" :items="getLocaleOptions()">
            </v-select>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {useI18n} from "vue-i18n";
import {useSettingsStore} from "@/stores/settings";

export default defineComponent({
  name: 'Settings',
  setup() {
    const i18n = useI18n();
    const t = i18n.t;
    const store = useSettingsStore();
    return {
      settings: {
        notifications: false,
      },
      i18n,
      getLocaleOptions: () => {
        return i18n.availableLocales.map(locale => {
          return {
            title: i18n.t('language.' + locale),
            value: locale,
          };
        });
      },
      store,
      localeStuff: {
        get locale() {
          return store.locale;
        },
        set locale(locale: string) {
          store.setLocale(locale);
        },
      },
      t
    }
  }
});
</script>