<template>
  <div class="flex flex-col gap-1">
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
  </div>
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
  <div>
    <Checkbox class="form-item" v-model="form.remember_me" binary />
    <label for="ingredient1"> Remember Me </label>
  </div>
  <Button class="form-item" type="submit" severity="secondary" label="Submit" @click="login" />
</template>
<script lang="ts">
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import { login } from '@/services/api/authService'

export default {
  // eslint-disable-next-line vue/no-reserved-component-names
  components: { Button, InputText, Checkbox },
  data() {
    return {
      form: {
        username: '',
        password: '',
        remember_me: false,
      },
    }
  },
  methods: {
    login() {
      login(this.form).then((data) => {
        localStorage.setItem('token', data.token)
        this.$router.push('/')
      })
    },
  },
}
</script>
