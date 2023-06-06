<template>
  <v-expansion-panels variant="popout">
    <v-expansion-panel title="Briefing" :disabled="!today(date) && !tomorrow(date)">
      <v-expansion-panel-text>
          <p style="align-content: center">
            <v-icon color="accent">mdi-alert-circle</v-icon>
            <a v-if="today(date)" href="https://www.skybriefing.com/o/dabs?today" target="_blank">DABS Briefing</a>
            <a v-if="tomorrow(date)" href="https://www.skybriefing.com/o/dabs?tomorrow" target="_blank">DABS Briefing</a>
          </p>
      </v-expansion-panel-text>
    </v-expansion-panel>
    <v-expansion-panel>
      <v-expansion-panel-title>Angemeldet ({{entries.length}})</v-expansion-panel-title>
      <v-expansion-panel-text>
        <v-list>
          <v-list-item v-for="entry in entries" :key="entry.name">
            <!-- Display your entry data here -->
            <b>({{ entry.entryType[0] }}) {{ entry.name }}</b>
            <p>{{ entry.message }}</p>
          </v-list-item>
        </v-list>
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script lang="ts">
import {defineComponent, onMounted, ref, toRefs, watch} from 'vue';
import {useStore} from '@/stores/reservation';
import {format, isToday, isTomorrow, parse, parseISO} from 'date-fns';
import type {RosterEntry} from "@/model";

export default defineComponent({
  props: {
    date: String,
  },
  setup(props) {
    const store = useStore();
    const {date} = toRefs(props);
    const entries = ref([] as RosterEntry[]);

    onMounted(async () => {
      if (!date.value) return;
      await store.getDay(date.value as string);
      entries.value = store.days[date.value] || [];
    });

    watch(date, (newDate) => {
      if (newDate) {
        store.getDay(newDate as string);
        entries.value = store.days[newDate] || [];
      }
    });

    const today = (date?: string) => date ? isToday(parse(date, 'yyyy-MM-dd', new Date())) : false

    const tomorrow = (date?: string) => date ? isTomorrow(parse(date, 'yyyy-MM-dd', new Date())) : false


    const formatDate = (date: string) => {
      if (!date) return '';
      return format(new Date(date), 'PPP');
    };

    return {
      entries,
      date,
      formatDate,
      today,
      tomorrow
    };
  },
});
</script>
