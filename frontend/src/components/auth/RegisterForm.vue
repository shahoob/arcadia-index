<template>
  <div>
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
    </div>
    <div class="flex flex-col gap-1">
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
    </div>
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
import { register, login } from '@/services/api/authService'
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
    // First register the user
    await register(form.value)
    
    // Then login to get tokens
    const loginData = await login({
      username: form.value.username,
      password: form.value.password,
      remember_me: true,
    })
    
    // Store tokens
    localStorage.setItem('token', loginData.token)
    localStorage.setItem('refreshToken', loginData.refresh_token)
    
    // Redirect to home
    router.push('/')
  } catch (error) {
    console.error('Registration failed:', error)
  }
}
</script>
