<template>
  <div class="min-h-screen bg-gradient-to-br from-[#0D1B2A] via-[#14213D] to-[#0D1B2A] flex items-center justify-center p-4">
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute top-20 left-20 w-64 h-64 bg-gradient-to-r from-[#6FFFE9] to-[#32FFC7] rounded-full opacity-20 blur-3xl animate-pulse"></div>
      <div class="absolute bottom-20 right-20 w-80 h-80 bg-gradient-to-r from-[#3ED9C9] to-[#6FFFE9] rounded-full opacity-15 blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
      <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-gradient-to-r from-[#32FFC7] to-[#3ED9C9] rounded-full opacity-10 blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
    </div>

    <div class="relative w-full max-w-md">
      <div class="bg-gradient-to-br from-[#14213D] to-[#0D1B2A] backdrop-blur-lg rounded-2xl shadow-2xl border border-[#32FFC7]/20 p-8">
        <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-[#6FFFE9] via-[#32FFC7] to-[#3ED9C9] opacity-20 blur-sm -z-10"></div>
        
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-white mb-2">
            {{ isLogin ? 'Welcome Back' : 'Create Account' }}
          </h1>
          <p class="text-[#6FFFE9] text-sm">
            {{ isLogin ? 'Sign in to your account' : 'Join us today' }}
          </p>
        </div>

        <div v-if="successMessage" class="mb-6 p-4 bg-green-900/30 border border-green-500/50 rounded-lg">
          <p class="text-green-400 text-sm text-center">{{ successMessage }}</p>
        </div>

        <div v-if="errors.general" class="mb-6 p-4 bg-red-900/30 border border-red-500/50 rounded-lg">
          <p class="text-red-400 text-sm text-center">{{ errors.general }}</p>
        </div>

        <div class="space-y-6">
          <div>
            <label for="email" class="block text-white text-sm font-medium mb-2">
              Email Address
            </label>
            <input
              type="email"
              id="email"
              v-model="email"
              @blur="validateEmailField"
              :class="[
                'w-full px-4 py-3 bg-[#0D1B2A]/80 border rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 transition-all duration-200',
                errors.email 
                  ? 'border-red-500 focus:ring-red-500/50' 
                  : 'border-[#32FFC7]/30 focus:ring-[#32FFC7]/50 focus:border-[#32FFC7]'
              ]"
              placeholder="Enter your email"
              :disabled="isLoading"
            />
            <p v-if="errors.email" class="mt-2 text-red-400 text-sm">{{ errors.email }}</p>
          </div>

          <div>
            <label for="password" class="block text-white text-sm font-medium mb-2">
              Password
            </label>
            <input
              type="password"
              id="password"
              v-model="password"
              :class="[
                'w-full px-4 py-3 bg-[#0D1B2A]/80 border rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 transition-all duration-200',
                errors.password 
                  ? 'border-red-500 focus:ring-red-500/50' 
                  : 'border-[#32FFC7]/30 focus:ring-[#32FFC7]/50 focus:border-[#32FFC7]'
              ]"
              placeholder="Enter your password"
              :disabled="isLoading"
            />
            
            <div v-if="!isLogin" class="mt-3">
              <p class="text-xs text-gray-400 mb-2">Password requirements:</p>
              <div class="space-y-1">
                <div class="flex items-center text-xs">
                  <div :class="[
                    'w-2 h-2 rounded-full mr-2',
                    password && validatePassword(password).minLength ? 'bg-green-500' : 'bg-gray-600'
                  ]"></div>
                  <span :class="password && validatePassword(password).minLength ? 'text-green-400' : 'text-gray-400'">
                    At least 8 characters
                  </span>
                </div>
                <div class="flex items-center text-xs">
                  <div :class="[
                    'w-2 h-2 rounded-full mr-2',
                    password && validatePassword(password).hasUpperCase ? 'bg-green-500' : 'bg-gray-600'
                  ]"></div>
                  <span :class="password && validatePassword(password).hasUpperCase ? 'text-green-400' : 'text-gray-400'">
                    One uppercase letter
                  </span>
                </div>
                <div class="flex items-center text-xs">
                  <div :class="[
                    'w-2 h-2 rounded-full mr-2',
                    password && validatePassword(password).hasLowerCase ? 'bg-green-500' : 'bg-gray-600'
                  ]"></div>
                  <span :class="password && validatePassword(password).hasLowerCase ? 'text-green-400' : 'text-gray-400'">
                    One lowercase letter
                  </span>
                </div>
                <div class="flex items-center text-xs">
                  <div :class="[
                    'w-2 h-2 rounded-full mr-2',
                    password && validatePassword(password).hasNumeric ? 'bg-green-500' : 'bg-gray-600'
                  ]"></div>
                  <span :class="password && validatePassword(password).hasNumeric ? 'text-green-400' : 'text-gray-400'">
                    One number
                  </span>
                </div>
                <div class="flex items-center text-xs">
                  <div :class="[
                    'w-2 h-2 rounded-full mr-2',
                    password && validatePassword(password).hasSpecialChar ? 'bg-green-500' : 'bg-gray-600'
                  ]"></div>
                  <span :class="password && validatePassword(password).hasSpecialChar ? 'text-green-400' : 'text-gray-400'">
                    One special character (!@#$%^&*...)
                  </span>
                </div>
              </div>
            </div>

            <div v-if="!isLogin && password && passwordStrength" class="mt-3">
              <div class="flex justify-between items-center mb-1">
                <span class="text-xs text-gray-400">Password strength:</span>
                <span :class="[
                  'text-xs font-medium',
                  passwordStrength.score <= 2 ? 'text-red-400' : 
                  passwordStrength.score <= 4 ? 'text-yellow-400' : 'text-green-400'
                ]">
                  {{ passwordStrength.label }}
                </span>
              </div>
              <div class="w-full bg-gray-700 rounded-full h-2">
                <div 
                  :class="['h-2 rounded-full transition-all duration-300', passwordStrength.color]"
                  :style="{ width: (passwordStrength.score / 5) * 100 + '%' }"
                ></div>
              </div>
            </div>
            
            <p v-if="errors.password" class="mt-2 text-red-400 text-sm">{{ errors.password }}</p>
          </div>

          <button
            @click="handleSubmit"
            :disabled="isLoading"
            class="w-full py-3 px-4 bg-gradient-to-r from-[#32FFC7] to-[#3ED9C9] text-[#0D1B2A] font-semibold rounded-lg shadow-lg hover:shadow-xl transform hover:scale-[1.02] transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
          >
            <div v-if="isLoading" class="flex items-center justify-center">
              <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-[#0D1B2A] mr-2"></div>
              Processing...
            </div>
            <span v-else>{{ isLogin ? 'Sign In' : 'Create Account' }}</span>
          </button>
        </div>

        <div class="mt-6 text-center">
          <p class="text-gray-400 text-sm">
            {{ isLogin ? "Don't have an account?" : "Already have an account?" }}
          </p>
          <button
            @click="toggleMode"
            class="mt-2 text-[#32FFC7] hover:text-[#6FFFE9] font-medium text-sm transition-colors duration-200"
            :disabled="isLoading"
          >
            {{ isLogin ? 'Create Account' : 'Sign In' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, watch } from 'vue' 
import { useRouter } from 'vue-router' 
export default {
  name: 'ImprovedRegistrationComponent',
  setup() {
    const router = useRouter()
    const isLogin = ref(true)
    const isLoading = ref(false)
    const email = ref('')
    const password = ref('')
    const errors = ref({})
    const successMessage = ref('')

    const validateEmail = (email) => {
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
      return emailRegex.test(email)
    }
    
    const validateEmailField = () => {
      if (!email.value) {
        errors.value.email = 'Email is required'
      } else if (!validateEmail(email.value)) {
        errors.value.email = 'Please enter a valid email address'
      } else {
        delete errors.value.email
      }
    }
    
    let emailDebounceTimer = null;
    watch(email, (newValue) => {
        clearTimeout(emailDebounceTimer);
        if (newValue) {
            emailDebounceTimer = setTimeout(() => {
                validateEmailField();
            }, 500); 
        } else {
            delete errors.value.email;
        }
    });


    const validatePassword = (password) => {
      const minLength = password.length >= 8
      const hasUpperCase = /[A-Z]/.test(password)
      const hasLowerCase = /[a-z]/.test(password)
      const hasNumeric = /\d/.test(password)
      const hasSpecialChar = /[!-+@#$%^&*(),.?":{}|<>]/.test(password)
      
      return {
        minLength,
        hasUpperCase,
        hasLowerCase,
        hasNumeric,
        hasSpecialChar,
        isValid: minLength && hasUpperCase && hasLowerCase && hasNumeric && hasSpecialChar
      }
    }

    const validateForm = () => {
      const newErrors = {}
      
      if (!email.value) {
        newErrors.email = 'Email is required'
      } else if (!validateEmail(email.value)) {
        newErrors.email = 'Please enter a valid email address'
      }
      
      if (!password.value) {
        newErrors.password = 'Password is required'
      } else if (!isLogin.value) {
        const passwordCheck = validatePassword(password.value)
        if (!passwordCheck.isValid) {
          newErrors.password = 'Please meet all password requirements listed above.'
        }
      }
      
      errors.value = newErrors
      return Object.keys(newErrors).length === 0
    }

    const handleSubmit = async () => {
      successMessage.value = ''
      
      validateEmailField();
      
      if (!validateForm()) return
      
      isLoading.value = true
      const endpoint = isLogin.value 
        ? 'http://localhost:3000/api/login' 
        : 'http://localhost:3000/api/register';

      try {
        const response = await fetch(endpoint, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            user_name: email.value,
            password: password.value
          })
        });

        if (!response.ok) {
          const errorData = await response.json().catch(() => ({ message: 'An unknown error occurred.' }));
          throw new Error(errorData.message || `Invalid username or password`);
        }
        if (isLogin.value) {
          successMessage.value = "Login successful! Welcome back."
          router.push('/home'); 
        } else {
          successMessage.value = 'Registration successful! You can now sign in.';
          isLogin.value = true;
        }
        email.value = ''
        password.value = ''
      } catch (error) {
        errors.value = { general: error.message || 'An error occurred. Please try again.' }
      } finally {
        isLoading.value = false
      }
    }

    const toggleMode = () => {
      isLogin.value = !isLogin.value
      errors.value = {}
      successMessage.value = ''
      email.value = ''
      password.value = ''
    }

    const passwordStrength = computed(() => {
      if (!password.value) return null
      const validation = validatePassword(password.value)
      const score = [
        validation.minLength,
        validation.hasUpperCase,
        validation.hasLowerCase,
        validation.hasNumeric,
        validation.hasSpecialChar
      ].filter(Boolean).length
      
      return {
        score,
        label: score <= 2 ? 'Weak' : score <= 4 ? 'Good' : 'Strong',
        color: score <= 2 ? 'bg-red-500' : score <= 4 ? 'bg-yellow-500' : 'bg-green-500'
      }
    })

    return {
      isLogin,
      isLoading,
      email,
      password,
      errors,
      successMessage,
      handleSubmit,
      toggleMode,
      passwordStrength,
      validatePassword,
      validateEmailField 
    }
  }
}
</script>

<style scoped>
@keyframes pulse {
  0%, 100% {
    opacity: 0.1;
  }
  50% {
    opacity: 0.3;
  }
}

.animate-pulse {
  animation: pulse 3s ease-in-out infinite;
}
</style>
