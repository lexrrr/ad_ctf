/** @type {import('../../../.svelte-kit/types/src/routes').LayoutServerLoad} */
import { env } from '$env/dynamic/public';

export async function load( {cookies, url}) {
    let username = undefined;
    const token = cookies.get('token')
    if (token) {
        const response = await fetch(env.PUBLIC_BACKEND_URL + '/username', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
        });

        if (response.ok) {
            username = await response.text();
            return {
                loggedIn: true,
                username: username
            };
        } else {
            console.error('Request failed');
        }
    } else {
        return {
            loggedIn: false,
            username: username
        };
    }
}