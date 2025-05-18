<template>
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="email"
      type="text"
      :placeholder="t('user.email')"
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
    :placeholder="t('user.username')"
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
      :placeholder="t('user.password')"
      v-model="form.password"
    />
    <!-- <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
        $form.email.error?.message
      }}</Message> -->
  </div>
  <div class="flex flex-col gap-1">
    <InputText
      class="form-item"
      name="password_verify"
      type="text"
      :placeholder="t('user.password_verify')"
      v-model="form.password_verify"
    />
    <!-- <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
        $form.email.error?.message
      }}</Message> -->
  </div>
  <Button
    class="form-item"
    type="submit"
    severity="secondary"
    :label="t('general.submit')"
    @click="handleRegister"
  />
</template>
<script setup lang="ts">
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { register } from '@/services/api/authService'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const form = ref({
  email: '',
  username: '',
  password: '',
  password_verify: '',
})

const router = useRouter()

const handleRegister = () => {
  register(form.value).then((data) => {
    localStorage.setItem('token', data.token)
    router.push('/login')
  })
}
</script>
