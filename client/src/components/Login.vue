<template>
    <v-dialog v-model="dialog" max-width="400px">
        <v-card class="bg-gray-700">
            <v-card-text class="pt-0 px-0">

                <v-card class="rounded-lg !bg-zinc-800">
                    <v-card-title class="text-center pa-4 mb-4">
                        <p class="text-xl my-2">Sign in to New Tab</p>
                        <p class="text-sm text-gray-200">Welcome back! Please sign in.</p>
                    </v-card-title>

                    <v-card-text class="text-center">

                        <div class="d-flex justify-space-around">
                            <v-btn class="px-10 !border-zinc-400" variant="outlined" @click="loginWithGoogle" aria-label="Login with Google" rounded>
                                <img src="/icons/google.svg" alt="Google" class="w-6 h-6">
                            </v-btn>

                            <v-btn class="px-10 !border-zinc-400" variant="outlined" @click="loginWithMicrosoft" aria-label="Login with Microsoft" rounded>
                                <img src="/icons/microsoft.svg" alt="Microsoft" class="w-6 h-6">
                            </v-btn>

                            <v-btn class="px-10 !border-zinc-400" variant="outlined" @click="loginWithGithub" aria-label="Login with Github" rounded>
                                <img src="/icons/github.svg" alt="GitHub" class="w-6 h-6">
                            </v-btn>
                        </div>
                    </v-card-text>

                    <v-divider class="my-4"> Or </v-divider>

                    <v-card-text>
                        <v-form ref="form" v-model="valid" lazy-validation>
                            <v-text-field v-model="email" :rules="emailRules" label="Email" required
                                prepend-inner-icon="mdi-email" variant="outlined" rounded></v-text-field>

                            <v-text-field v-model="password" :rules="passwordRules" label="Password" type="password"
                                required prepend-inner-icon="mdi-lock" autocomplete="password" variant="outlined" rounded></v-text-field>

                            <v-btn block class="mt-4 bg-white" :disabled="!valid" @click="login" rounded>
                                Login
                            </v-btn>
                        </v-form>
                    </v-card-text>

                </v-card>
            </v-card-text>

            <v-card-text class="pt-0">
                <p class="text-center">Don't have an account? <strong>Sign up here.</strong></p>
            </v-card-text>

        </v-card>

    </v-dialog>
</template>

<script setup>
import { ref, reactive } from 'vue';

const dialog = ref(true);
const valid = ref(true);
const email = ref('');
const password = ref('');
const form = ref(null);

const emailRules = [
    v => !!v || 'Email is required',
    v => /.+@.+\..+/.test(v) || 'Email must be valid',
];

const passwordRules = [
    v => !!v || 'Password is required',
    v => v.length >= 6 || 'Password must be at least 6 characters',
];

const open = () => {
    dialog.value = true;
};

const close = () => {
    dialog.value = false;
};

const login = () => {
    if (form.value.validate()) {
        // Implement login logic here
        console.log('Logging in with email and password');
    }
};

const loginWithGoogle = () => {
    // Implement Google OAuth login
    console.log('Logging in with Google');
};

const loginWithMicrosoft = () => {
    // Implement Microsoft OAuth login
    console.log('Logging in with Microsoft');
};

const loginWithGithub = () => {
    // Implement GitHub OAuth login
    console.log('Logging in with GitHub');
};

defineExpose({ open, close });
</script>

<style scoped>
.v-card {
    border-radius: 12px;
}
</style>