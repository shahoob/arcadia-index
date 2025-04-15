<template>
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="email"
      type="text"
      placeholder="Email"
      v-model="form.email"
    />
    <!-- <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{
        $form.username.error?.message
      }}</Message> -->
  </div>
  <InputText
    class="form-item"
    name="username"
    type="text"
    placeholder="Username"
    v-model="form.username"
  />
  <!-- <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{
        $form.username.error?.message
      }}</Message> -->
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="password"
      type="text"
      placeholder="Password"
      v-model="form.password"
    />
    <!-- <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
        $form.email.error?.message
      }}</Message> -->
  </div>
  <Button class="form-item" type="submit" severity="secondary" label="Submit" @click="login" />
</template>
<script lang="ts">
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import { register } from '@/services/api/authService'

export default {
  components: { Button, InputText },
  data() {
    return {
      form: {
        email: '',
        username: '',
        password: '',
      },
    }
  },
  methods: {
    login() {
      register(this.form).then((data) => {
        localStorage.setItem('token', data.token)
        this.$router.push('/')
      })
    },
  },
}
</script>
