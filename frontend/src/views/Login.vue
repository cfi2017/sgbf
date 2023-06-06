<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12" sm="8" md="6" lg="4">
        <v-form @submit="handleSubmit">
          <v-text-field
              v-model="form.username"
              label="Username"
              required
              v-validate="'required'"
              name="username"
              :error-messages="errors.username"
          ></v-text-field>

          <v-text-field
              v-model="form.password"
              type="password"
              label="Password"
              required
              v-validate="'required'"
              name="password"
              :error-messages="errors.password"
          ></v-text-field>

          <v-checkbox
              v-model="form.rememberMe"
              label="Remember me"
          ></v-checkbox>

          <v-btn type="submit">Login</v-btn>
        </v-form>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent, reactive, ref } from 'vue';
import { useStore } from '@/stores/reservation';
import { useI18n } from 'vue-i18n';
import { useForm } from 'vee-validate';
import router from "@/router";

export default defineComponent({
  setup() {
    const store = useStore();
    const i18n = useI18n();

    const form = reactive({
      username: '',
      password: '',
      rememberMe: false,
    });

    const { handleSubmit, errors } = useForm();

    const onSubmit = async (values: any) => {
      if (form.rememberMe) {
        // Save credentials to local storage
        localStorage.setItem('username', form.username);
        localStorage.setItem('password', form.password);
      }

      // Attempt to login
      await store.login(form.username, form.password);
      await router.push('/reservation/calendar');
    };

    return {
      form,
      errors,
      i18n,
      handleSubmit: handleSubmit(onSubmit),
    };
  },
});
</script>
