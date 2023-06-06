<template>
  <v-container style="max-width: 800px">
    <v-btn @click="refreshData">Refresh</v-btn>
    <v-list two-line subheader>
      <v-card-subtitle>Calendar</v-card-subtitle>
      <v-list density="compact">
        <template v-for="day in days" :key="day.date">

        <v-list-item>
          <template v-slot:append>
            <v-btn
                variant="text"
                icon="mdi-eye"
                @click="selectDay(day.date)"
            ></v-btn>
          </template>
          <v-list-item-title>{{ formatDate(day.date) }}</v-list-item-title>
          <v-list-item-subtitle>
            Registered pilots: {{ day.registeredPilots?.definitive }} definitive,
            {{ day.registeredPilots?.tentative }} tentative
          </v-list-item-subtitle>
        </v-list-item>

        <v-list-item v-for="entry in day.entries" :key="entry.name">
          <v-list-item-title>
            <template v-if="entry.entryType === 'FlightInstructor'"><v-icon color="primary">mdi-school</v-icon> Schule</template>
            <template v-if="entry.entryType === 'WinchOperator'"><v-icon color="primary">mdi-airplane-takeoff</v-icon> Winde</template>
            <template v-if="entry.entryType === 'TowingPilot'"><v-icon color="primary">mdi-airplane-takeoff</v-icon> Schlepp</template>
            : {{ entry.name }}
          </v-list-item-title>
          <v-list-item-subtitle>{{ entry.timeFrame[0] }} - {{ entry.timeFrame[1] }}</v-list-item-subtitle>
          <v-list-item-subtitle v-if="entry.note1">Note 1: {{ entry.note1 }}</v-list-item-subtitle>
          <v-list-item-subtitle v-if="entry.note2">Note 2: {{ entry.note2 }}</v-list-item-subtitle>
        </v-list-item>
        <v-divider></v-divider>
        </template>
      </v-list>
    </v-list>
    <v-dialog v-model="showDialog" style="max-width: 800px">
      <v-card>
        <v-card-title>
          {{ formatDate(selectedDay) }}
        </v-card-title>
        <v-card-text>
          <reservation-day :date="selectedDay"></reservation-day>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="primary" @click="showDialog = false">
            Close
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>


<script lang="ts">
import {computed, defineComponent, onMounted, ref} from 'vue';
import {useStore} from '@/stores/reservation';
import {format} from 'date-fns';
import ReservationDay from "@/components/ReservationDay.vue";

export default defineComponent({
  name: 'Calendar',
  components: {ReservationDay},
  setup() {
    const store = useStore();
    const selectedDay = ref('');
    const showDialog = ref(false);

    onMounted(async () => {
      await store.getCalendar();
    });
    const refreshData = async () => {
      await store.getCalendar();
    };

    const formatDate = (date: string) => {
      return format(new Date(date), 'PPP');  // Pretty print the date
    };
    const selectDay = (date: string) => {
      selectedDay.value = date;
      showDialog.value = true;
    }

    return {
      days: computed(() => store.calendar),
      refreshData,
      formatDate,
      selectDay,
      selectedDay,
      showDialog
    };
  },
});
</script>
