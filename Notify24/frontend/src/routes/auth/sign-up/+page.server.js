/** @type {import('./$types').Actions} */

import {redirect} from "@sveltejs/kit";
import { env } from '$env/dynamic/public';

export const actions = {
    register: async ({ cookies, request }) => {
        const data = await request.formData();
        const email = data.get('email');
        const password = data.get('password');
        const passwordConfirmation = data.get('password-confirmation');

        if (password !== passwordConfirmation) {
            return { success: false, message: 'Passwords do not match' };
        }

        const response = await fetch(env.PUBLIC_BACKEND_URL + '/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({"password": password, "email": email})
        });

        if (response.ok) {
            const responseJson = await response.json();
            cookies.set('token', responseJson['accessToken'], {path: '/', secure: false});
            redirect(303, '/');
        } else {
            console.error('Request failed');
        }
    },
};