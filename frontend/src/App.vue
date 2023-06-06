
<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
      <v-toolbar-title>SGBF Reservationssystem</v-toolbar-title>
      <v-spacer></v-spacer>
      <!-- Add more navigation items here -->
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" app temporary>
      <!-- Add navigation items here -->
    </v-navigation-drawer>

    <v-main style="max-width: 90%">
      <router-view></router-view>
    </v-main>

    <v-footer app>
      <!-- Add footer items here -->
    </v-footer>
  </v-app>
</template>

<script lang="ts">
import { ref } from 'vue';

export default {
  name: 'App',
  setup() {
    const drawer = ref(false);

    return {
      drawer,
    };
  },
};
</script>

<style scoped>
@media (min-width: 600px) {
  .v-navigation-drawer--temporary {
    width: 300px;
  }
}
</style>
