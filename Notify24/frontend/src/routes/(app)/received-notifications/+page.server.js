/** @type {import('../../../.svelte-kit/types/src/routes').PageServerLoad} */
import { env } from '$env/dynamic/public';

export async function load({cookies, url}) {
    let notifications = [];

    const response = await fetch(env.PUBLIC_BACKEND_URL + '/notification/all', {
        method: 'GET'
    })
        .then(response => response.json())
        .then(json => {
            notifications = json;
        })
        .catch(error => console.log('Error:', error))
    return { 'notifications': notifications }
}