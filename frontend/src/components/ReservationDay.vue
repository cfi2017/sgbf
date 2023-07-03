<template>
  <v-expansion-panels variant="popout">
    <v-expansion-panel :title="t('briefing.title')" :disabled="!today(date) && !tomorrow(date)">
      <v-expansion-panel-text>
          <p style="align-content: center">
            <v-icon color="accent">mdi-alert-circle</v-icon>
            <a v-if="today(date)" href="https://www.skybriefing.com/o/dabs?today" target="_blank">{{t('briefing.dabs')}}</a>
            <a v-if="tomorrow(date)" href="https://www.skybriefing.com/o/dabs?tomorrow" target="_blank">{{t('briefing.dabs')}}</a>
          </p>
      </v-expansion-panel-text>
    </v-expansion-panel>
    <v-expansion-panel>
      <v-expansion-panel-title>{{ t('registered.title') }} ({{day?.entries?.length || 0}})</v-expansion-panel-title>
      <v-expansion-panel-text>
        <v-list>
          <v-list-subheader>{{ t('registered.definite') }}</v-list-subheader>
          <v-list-item v-for="entry in day?.entries.filter(e => e.entryType == RosterEntryType.Definite) || []" :key="entry.name">
            <!-- Display your entry data here -->
            <b>{{ entry.name }}</b>
            <p>{{ entry.message }}</p>
          </v-list-item>
          <v-list-subheader>{{ t('registered.tentative') }}</v-list-subheader>
          <v-list-item v-for="entry in day?.entries.filter(e => e.entryType == RosterEntryType.Tentative) || []" :key="entry.name">
            <!-- Display your entry data here -->
            <b>{{ entry.name }}</b>
            <p>{{ entry.message }}</p>
          </v-list-item>
          <v-list-subheader>{{ t('registered.unavailable') }}</v-list-subheader>
          <v-list-item v-for="entry in day?.entries.filter(e => e.entryType == RosterEntryType.Unavailable) || []" :key="entry.name">
            <!-- Display your entry data here -->
            <b>{{ entry.name }}</b>
            <p>{{ entry.message }}</p>
          </v-list-item>
        </v-list>
      </v-expansion-panel-text>
    </v-expansion-panel>
    <v-expansion-panel>
      <v-expansion-panel-title>{{ t('registration.title') }}</v-expansion-panel-title>
      <v-expansion-panel-text>
        <v-form @submit.prevent @submit="submit">
          <v-select v-model="entryType.value.value"
                    :error-messages="entryType.errorMessage.value || []"
                    :items="[{title: t('registration.definite'), value: 'Definite'}, {title: t('registration.tentative'), value: 'Tentative'}, {title: t('registration.unavailable'), value: 'Unavailable'}]"
                    :label="t('registration.entry_type')"
          ></v-select>
          <v-textarea v-model="remarks.value.value"
                      :error-messages="remarks.errorMessage.value || []"
                      :label="t('registration.remarks')"
                      rows="4"></v-textarea>
          <v-btn color="primary" type="submit">{{ t('registration.save') }}</v-btn>
        </v-form>
      </v-expansion-panel-text>
    </v-expansion-panel>
    <v-expansion-panel>
      <v-expansion-panel-title>{{ t('reservations.title') }}</v-expansion-panel-title>
      <v-expansion-panel-text>
        <v-list>
          <v-list-item v-for="reservation in reservations()" :key="reservation.plane">
            <!-- Display your reservation data here -->
            <b>{{ reservation.plane }}</b>
            <p>{{ reservation.reservedBy }}</p>
            <p v-for="comment in reservation.comments" :key="comment">{{ comment }}</p>
          </v-list-item>
        </v-list>
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script lang="ts">
import {defineComponent, onMounted, ref, toRefs, watch} from 'vue';
import {useStore} from '@/stores/reservation';
import {format, isToday, isTomorrow, parse} from 'date-fns';
import type {Day, Reservation} from "@/model";
import {useField, useForm} from "vee-validate";
import {RosterEntryType} from "@/model";
import {useI18n} from "vue-i18n";

export default defineComponent({
  computed: {
    RosterEntryType() {
      return RosterEntryType
    }
  },
  props: {
    date: String,
  },
  setup(props) {
    const store = useStore();
    const {date} = toRefs(props);
    const day = ref({} as Day);

    const {handleSubmit, handleReset} = useForm({
      validationSchema: {
        entryType: (value) => !!value || 'Bitte wähle eine Option',
        // remarks: (value) => !!value || 'Bitte gib einen Kommentar ein',
      }
    });

    const entryType = useField('entryType');
    const remarks = useField('remarks');


    onMounted(async () => {
      if (!date.value) return;
      await store.getDay(date.value as string);
      day.value = store.days[date.value];
      entryType.value.value = day.value?.entryType || '';
      remarks.value.value = day.value?.remarks || '';
    });

    watch(date, (newDate) => {
      if (newDate) {
        store.getDay(newDate as string);
        day.value = store.days[newDate];
        entryType.value.value = day.value?.entryType || '';
        remarks.value.value = day.value?.remarks || '';
      }
    });

    const today = (date?: string) => date ? isToday(parse(date, 'yyyy-MM-dd', new Date())) : false

    const tomorrow = (date?: string) => date ? isTomorrow(parse(date, 'yyyy-MM-dd', new Date())) : false


    const formatDate = (date: string) => {
      if (!date) return '';
      return format(new Date(date), 'PPP');
    };

    const {t} = useI18n();

    return {
      day,
      date,
      formatDate,
      today,
      tomorrow,
      entryType,
      remarks,
      reservations: () => day.value?.reservations || [] as Reservation[],
      submit: handleSubmit(async values => {
        console.log('submitted');
        console.log(day.value);
        console.log(values);
        await store.updateDay(date.value as string, values.entryType, values.remarks);
      }),
      t: (key: string) => t('reservation.day.' + key),
    };
  },
});
</script>
