<script setup lang="ts">
import {ref} from "vue";
import {VDataTable} from 'vuetify/labs/VDataTable'
import type {Start} from "@/model";
import {startListStore} from "@/stores/startlist";

type UnwrapReadonlyArrayType<A> = A extends Readonly<Array<infer I>> ? UnwrapReadonlyArrayType<I> : A
type DT = InstanceType<typeof VDataTable>;
type ReadonlyDataTableHeader = UnwrapReadonlyArrayType<DT['headers']>;

const store = startListStore();

const forms = store.forms;

const headers = [
  {
    title: "Pilot",
    align: "start",
    sortable: false,
    key: "pic"
  },
  {
    title: "Copilot",
    align: "start",
    sortable: false,
    key: "copilot"
  },
  {
    title: "Plane",
    align: "start",
    sortable: false,
    key: "plane"
  },
  {
    title: "From",
    align: "start",
    sortable: false,
    key: "from"
  },
  {
    title: "To",
    align: "start",
    sortable: false,
    key: "to"
  },
] as ReadonlyDataTableHeader[];

const startlist = ref([] as Start[]);

const editItem = (item) => {
  console.log(item);
};

const deleteItem = (item) => {
  console.log(item);
};

const add = (item: Start) => {
  store.addStart(item);
  item.from = '';
  item.to = '';
}

const setTime = (obj: any, key: string) => {
  const date = new Date();
  const value = `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
  obj[key] = value;
}

const online = ref(navigator.onLine);

addEventListener('online', () => online.value = true);
addEventListener('offline', () => online.value = false);

const edit = (item) => {

};

</script>

<template>
  <v-card>
    <v-card-title class="d-flex justify-space-between">
      <span>Startliste</span>
      <v-btn :disabled="!online" @click="store.save()">Sync</v-btn>
    </v-card-title>
    <v-card-item>
      <div>
        <div v-for="form in ['a', 'b']" class="d-md-flex justify-md-space-between align-center">
          <v-autocomplete label="PIC" v-model="forms[form].pic" :items="store.pilots" hide-details="auto" class="mr-1 ml-1"></v-autocomplete>
          <v-autocomplete label="Copilot" v-model="forms[form].copilot" :items="store.pilots" v-if="!forms[form].isPax" hide-details="auto" class="mr-1 ml-1"></v-autocomplete>
          <v-text-field label="PAX" v-model="forms[form].copilot" v-if="forms[form].isPax" hide-details="auto" class="mr-1 ml-1"></v-text-field>
          <v-checkbox v-model="forms[form].isPax" label="PAX" hide-details="auto" class="mr-1 ml-1"></v-checkbox>
          <v-select label="Plane" v-model="forms[form].plane"
                    :items="['BF2', 'BF3']"
                    hide-details="auto" class="mr-1 ml-1"></v-select>
          <v-text-field label="From" v-model="forms[form].from" hide-details="auto" class="mr-1 ml-1">
            <template v-slot:append-inner>
              <v-btn icon="mdi-airplane-clock" size="small" @click="setTime(forms[form], 'from')"></v-btn>
            </template>
          </v-text-field>
          <v-text-field label="To" v-model="forms[form].to" hide-details="auto" class="mr-1 ml-1">
            <template v-slot:append-inner>
              <v-btn icon="mdi-airplane-clock" size="small" @click="setTime(forms[form], 'to')"></v-btn>
            </template>
          </v-text-field>
          <v-btn prepend-icon="mdi-send" @click="add(forms[form])">Add</v-btn>
        </div>
      </div>
      <v-divider class="pt-3"></v-divider>
      <v-data-table-virtual
          :headers="headers"
          :items="store.list"
          height="400"
          item-value="pic"
          class="elevation-1"
      >

        <template v-slot:item.actions="{item}">
          <v-icon size="small" class="me-2" @click="edit(item)">mdi-pencil</v-icon>
        </template>
      </v-data-table-virtual>
    </v-card-item>
  </v-card>
</template>

<style scoped>

</style>