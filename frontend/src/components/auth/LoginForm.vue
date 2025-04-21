<template>
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="username"
      type="text"
      :placeholder="$t('user.username')"
      v-model="form.username"
    />
  </div>
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="password"
      type="text"
      :placeholder="$t('user.password')"
      v-model="form.password"
    />
  </div>
  <div>
    <Checkbox class="form-item" v-model="form.remember_me" binary />
    <label for="ingredient1"> {{ $t('auth.remember_me') }} </label>
  </div>
  <Button class="form-item" type="submit" severity="secondary" label="Submit" @click="login" />
</template>
<script lang="ts">
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import { login } from '@/services/api/authService'
import { useUserStore } from '@/stores/user'

export default {
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
        localStorage.setItem('user', JSON.stringify(data.user))
        const userStore = useUserStore()
        userStore.setUser(data.user)
        this.$router.push('/')
      })
    },
  },
}
</script>
