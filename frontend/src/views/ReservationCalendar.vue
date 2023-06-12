<template>
  <v-container style="max-width: 800px">
    <v-card>
      <v-card-title>
        {{ t('reservation.title') }} SGBF
      </v-card-title>
      <v-card-actions>
        <v-btn variant="tonal" :disabled="isLoading" prepend-icon="mdi-refresh" color="primary" @click="refreshData">{{ t('reservation.refresh') }}</v-btn>
        <v-progress-circular v-if="isLoading" indeterminate></v-progress-circular>
      </v-card-actions>
    </v-card>
    <v-divider></v-divider>
    <v-list two-line subheader>
      <v-card-subtitle>{{ t('reservation.calendar') }}</v-card-subtitle>
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
          <v-list-item-title><b>{{ formatDate(day.date) }}</b></v-list-item-title>
          <v-list-item-subtitle>
            {{ t('reservation.entry.registered_label') }}: {{ day.registeredPilots?.definitive }} {{ t('reservation.entry.definite') }},
            {{ (day.registeredPilots?.tentative || 0) - (day.registeredPilots?.definitive || 0) }} {{ t('reservation.entry.tentative') }}
          </v-list-item-subtitle>
        </v-list-item>

        <v-list-item v-for="entry in day.entries" :key="entry.name">
          <v-list-item-title>
            <template v-if="entry.entryType === 'FlightInstructor'"><v-icon color="primary">mdi-school</v-icon>
              {{ t('reservation.entry.pilotType.instructor') }}</template>
            <template v-if="entry.entryType === 'WinchOperator'"><v-icon color="primary">mdi-airplane-takeoff</v-icon> {{ t('reservation.entry.pilotType.winchOperator') }}</template>
            <template v-if="entry.entryType === 'TowingPilot'"><v-icon color="primary">mdi-airplane-takeoff</v-icon> {{ t('reservation.entry.pilotType.towPilot') }}</template>: {{ entry.name }}
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
            {{t('reservation.day.close')}}
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
import {useI18n} from "vue-i18n";

export default defineComponent({
  name: 'Calendar',
  components: {ReservationDay},
  setup() {
    const store = useStore();
    const selectedDay = ref('');
    const showDialog = ref(false);
    const isLoading = ref(true);

    onMounted(async () => {
      isLoading.value = true;
      await store.getCalendar();
      isLoading.value = false;
    });
    const refreshData = async () => {
      isLoading.value = true;
      await store.getCalendar();
      isLoading.value = false;
    };

    const formatDate = (date: string) => {
      return format(new Date(date), 'PPPP');  // Pretty print the date
    };
    const selectDay = (date: string) => {
      selectedDay.value = date;
      showDialog.value = true;
    }

    const {t} = useI18n();
    return {
      days: computed(() => store.calendar),
      refreshData,
      formatDate,
      selectDay,
      selectedDay,
      showDialog,
      isLoading,
      t
    };
  },
});
</script>
