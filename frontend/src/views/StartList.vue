<script setup lang="ts">
import {ref} from "vue";

const headers = ref([{
  title: "Pilot",
  align: "start",
  sortable: false,
  key: "pilot"
}]);
const startlist = ref([]);

const editItem = (item) => {
  console.log(item);
};

const deleteItem = (item) => {
  console.log(item);
};
</script>

<template>
  <v-card>
    <v-card-title class="d-flex justify-space-between">
      <span>Startliste</span>
      <span>Verbunden mit SGBF Starttracker</span>
    </v-card-title>
    <v-card-item>
      <v-data-table :headers="headers as any" :items="startlist" :items-per-page="10" class="elevation-1">
        <template v-slot:item.action="{ item }">
          <v-icon small class="mr-2" @click="editItem(item)">mdi-pencil</v-icon>
          <v-icon small @click="deleteItem(item)">mdi-delete</v-icon>
        </template>
      </v-data-table>
    </v-card-item>
  </v-card>
</template>

<style scoped>

</style>