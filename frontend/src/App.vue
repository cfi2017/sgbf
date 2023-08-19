<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
      <v-toolbar-title>SGBF {{t('reservation.title')}}</v-toolbar-title>
      <v-spacer></v-spacer>
      <!-- Add more navigation items here -->
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" app temporary>
      <!-- Add navigation items here -->
      <v-list>
        <v-list-item prepend-icon="mdi-airplane-clock" @click="router.push('/reservation/calendar')" :title="t('nav.reservation')"></v-list-item>
        <v-list-item v-if="dev" prepend-icon="mdi-clipboard-check-outline" @click="router.push('/reservation/checklist')" :title="t('nav.checklist')"></v-list-item>
        <v-list-item v-if="dev" prepend-icon="mdi-airplane-takeoff" @click="router.push('/startlist')" :title="t('nav.startlist')"></v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-cog" :title="t('nav.settings')" @click="router.push('/settings')"></v-list-item>
        <v-list-item prepend-icon="mdi-exit-to-app" :title="t('nav.logout')" @click="logout()"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main style="max-width: 100%">
      <router-view></router-view>
    </v-main>

    <v-footer app>
      <!-- Add footer items here -->
    </v-footer>
  </v-app>
</template>

<script lang="ts">
import {defineComponent, ref} from 'vue';
import router from "@/router";
import {useI18n} from "vue-i18n";
import {useSettingsStore} from "@/stores/settings";
import {useStore} from "@/stores/reservation";
export default defineComponent({
  name: 'App',
  setup() {
    const store = useStore();
    const logout = () => {
      store.logout();
    }
    const {t} = useI18n();
    const settings = useSettingsStore();
    store.checkLogin().then(() => console.log('checked login'))

    // get store.dev reactive
    const dev = store.dev;

    settings.loadSettings();
    const drawer = ref(false);
    return {
      drawer,
      router,
      logout,
      dev,
      t
    }
  }
})
</script>

<style scoped>
@media (min-width: 600px) {
  .v-navigation-drawer--temporary {
    width: 300px;
  }
}
</style>
