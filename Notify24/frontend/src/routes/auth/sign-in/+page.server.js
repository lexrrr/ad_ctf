/** @type {import('./$types').Actions} */

import {fail, redirect} from "@sveltejs/kit";
import { env } from '$env/dynamic/public';

export const actions = {
    login: async ({ cookies, request }) => {
        const data = await request.formData();
        const email = data.get('email');
        const password = data.get('password');


        const response = await fetch(env.PUBLIC_BACKEND_URL + '/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({"password": password, "email": email})
        });

        if (response.ok) {
            const responseString = await response.text();
            cookies.set('token', responseString, {path: '/', secure: false});
            redirect(303, '/?loggedIn=true');
        }
        else if (response.status === 401) {
            return fail(400, { incorrect: 'Invalid credentials'})
        }
        else {
            console.error('Request failed');
        }
    },
};