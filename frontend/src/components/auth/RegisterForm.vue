<template>
  <div class="form">
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
    <Password
      class="form-item"
      name="password"
      v-model="form.password"
      :placeholder="t('user.password')"
      :feedback="false"
      toggleMask
    />
    <!-- <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
          $form.email.error?.message
        }}</Message> -->
    <Password
      class="form-item"
      name="password_verify"
      v-model="form.password_verify"
      :placeholder="t('user.password_verify')"
      :feedback="false"
      toggleMask
    />
    <!-- <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
          $form.email.error?.message
        }}</Message> -->
    <Button
      class="form-item w-full"
      type="submit"
      severity="secondary"
      :label="t('general.submit')"
      @click="handleRegister"
    />
  </div>
</template>
<script setup lang="ts">
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
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

const handleRegister = async () => {
  try {
    // Register the user
    await register(form.value)

    // Redirect to login page after successful registration
    router.push('/login')
  } catch (error) {
    console.error('Registration failed:', error)
  }
}
</script>
<style scoped>
.form {
  display: flex;
  flex-direction: column;
}
</style>
