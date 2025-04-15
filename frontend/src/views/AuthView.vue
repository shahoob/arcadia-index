<template>
  <div class="auth-form">
    <LoginForm v-if="$router.currentRoute.value.path.includes('login')" />
    <RegisterForm v-if="$router.currentRoute.value.path.includes('register')" />
  </div>
</template>
<script lang="ts">
import LoginForm from '@/components/auth/LoginForm.vue'
import RegisterForm from '@/components/auth/RegisterForm.vue'
export default {
  components: { LoginForm, RegisterForm },
}
</script>

<style>
.auth-form {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.form-item {
  margin-top: 10px;
}
</style>
