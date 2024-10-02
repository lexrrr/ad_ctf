import { env } from '$env/dynamic/public';
import {redirect} from "@sveltejs/kit";

export async function load( {cookies, url}) {
    let username = undefined;
    const token = cookies.get('token')
    let ipSetId = url.searchParams.get("ipSetId")

    if (token) {
        const response = await fetch(env.PUBLIC_BACKEND_URL + '/ip-set/' + ipSetId, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
        });

        if (response.ok) {
            let ipSet = await response.json();
            return ipSet;
        }
    }
}

/** @type {import('./$types').Actions} */
export const actions = {
    send: async ({ cookies, request }) => {
        const data = await request.formData();
        const notification = data.get('notification');
        const notificationEncoded = encodeURIComponent(notification)
        const ipSetId = data.get('ipSetId');

        const token = cookies.get('token')

        const controller = new AbortController()
        let notificationResponses = [];

        let sendUrl = env.PUBLIC_BACKEND_URL + `/notification/send?ipSetId=${ipSetId}&notification=${notificationEncoded}`

        const response = await fetch(sendUrl,  {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            signal: controller.signal
        })
            .then(response => response.json())
            .then(json => {
                notificationResponses = json;
            })
            .catch(error => console.log('Error:', error))
        return { 'response': notificationResponses }
    },
};